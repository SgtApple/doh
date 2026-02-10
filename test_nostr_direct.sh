#!/bin/bash
# Manual Nostr posting test with debug output

echo "=== Nostr Manual Test ==="
echo ""

echo "1. Checking credentials..."
CREDS=$(secret-tool lookup service com.sgtapple.doh username credentials 2>&1)

if [ -z "$CREDS" ]; then
    echo "   ❌ NO CREDENTIALS FOUND!"
    echo "   Please configure in Doh Settings first"
    exit 1
fi

echo "   ✅ Credentials found"
echo ""

echo "2. Checking Nostr configuration..."
NSEC=$(echo "$CREDS" | grep -o '"nostr_nsec":"[^"]*"' | cut -d'"' -f4)
USE_PLEB=$(echo "$CREDS" | grep -o '"nostr_use_pleb_signer":[^,]*' | cut -d':' -f2)
RELAYS=$(echo "$CREDS" | grep -o '"nostr_relays":\[[^]]*\]' | cut -d':' -f2-)

if [ "$USE_PLEB" == "true" ]; then
    echo "   Mode: Pleb_Signer"
    echo "   ⚠️  WARNING: Pleb_Signer has issues. Try nsec mode instead!"
else
    echo "   Mode: Direct nsec"
    if [ -z "$NSEC" ] || [ "$NSEC" == "null" ]; then
        echo "   ❌ No nsec key found!"
        echo "   Please add your nsec in Doh Settings"
        exit 1
    else
        echo "   ✅ nsec key present (length: ${#NSEC})"
        if [[ ! "$NSEC" =~ ^nsec1 ]]; then
            echo "   ⚠️  WARNING: Key doesn't start with 'nsec1'!"
        fi
    fi
fi

echo "   Relays: $RELAYS"
echo ""

echo "3. Testing with Rust directly..."
cd /home/sgtapple/Projects/doh/doh

# Create a simple test program
cat > /tmp/test_nostr.rs << 'RUST_EOF'
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read credentials
    let creds_json = std::fs::read_to_string("/tmp/test_creds.json")?;
    let creds: serde_json::Value = serde_json::from_str(&creds_json)?;
    
    let nsec = creds["nostr_nsec"].as_str()
        .ok_or("No nsec key")?;
    
    println!("Parsing nsec key...");
    let keys = Keys::parse(nsec)?;
    println!("✅ Keys parsed successfully");
    println!("Public key: {}", keys.public_key());
    
    // Create client
    let client = Client::new(keys);
    
    // Add default relays
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://relay.nostr.band").await?;
    
    println!("Connecting to relays...");
    client.connect().await;
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Try to post
    println!("Creating test note...");
    let builder = EventBuilder::text_note("Test from Doh debug script", vec![]);
    
    println!("Sending to relays...");
    let event_id = client.send_event_builder(builder).await?;
    println!("✅ Posted successfully!");
    println!("Event ID: {}", event_id);
    
    Ok(())
}
RUST_EOF

# Save credentials to temp file
echo "$CREDS" > /tmp/test_creds.json

echo "4. Running test post..."
echo ""

if rustc --edition 2021 /tmp/test_nostr.rs -o /tmp/test_nostr 2>&1 | head -20; then
    /tmp/test_nostr 2>&1
else
    echo "Compilation failed. Need to add dependencies."
    echo ""
    echo "Instead, let's check if doh can load the credentials:"
    echo ""
    RUST_LOG=debug /usr/bin/doh 2>&1 | head -20 &
    DPID=$!
    sleep 3
    kill $DPID 2>/dev/null
fi

echo ""
echo "=== Test Complete ==="
echo ""
echo "Next steps:"
echo "1. If keys parsed successfully, the problem is in the posting logic"
echo "2. If parsing failed, the nsec format is wrong"
echo "3. Try posting via Doh UI and check: journalctl --user -f"
RUST_EOF

chmod +x /tmp/test_nostr.sh
/tmp/test_nostr.sh
