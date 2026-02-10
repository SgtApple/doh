#!/usr/bin/env python3
"""
Helper script to set up test credentials for Doh applet.
This will store credentials in the system keyring.
"""

import json
import subprocess
import sys

def setup_credentials():
    """Set up test credentials for Doh."""
    
    print("=" * 60)
    print("Doh! Credential Setup")
    print("=" * 60)
    print()
    print("This script will help you set up credentials for testing.")
    print("Credentials are stored securely in your system keyring.")
    print()
    
    credentials = {
        "twitter_consumer_key": None,
        "twitter_consumer_secret": None,
        "twitter_access_token": None,
        "twitter_access_secret": None,
        "bluesky_handle": None,
        "bluesky_app_password": None,
        "nostr_nsec": None,
        "nostr_use_pleb_signer": False,
        "nostr_image_host_url": None,
        "nostr_relays": [],
        "threads_access_token": None,
        "threads_user_id": None,
    }
    
    print("\n--- BlueSky Configuration ---")
    setup_bluesky = input("Configure BlueSky? (y/n): ").lower() == 'y'
    if setup_bluesky:
        credentials["bluesky_handle"] = input("BlueSky handle (e.g., user.bsky.social): ").strip()
        credentials["bluesky_app_password"] = input("BlueSky app password: ").strip()
    
    print("\n--- Nostr Configuration ---")
    setup_nostr = input("Configure Nostr? (y/n): ").lower() == 'y'
    if setup_nostr:
        use_pleb = input("Use Pleb_Signer? (y/n): ").lower() == 'y'
        credentials["nostr_use_pleb_signer"] = use_pleb
        
        if not use_pleb:
            credentials["nostr_nsec"] = input("Nostr nsec key: ").strip()
        
        use_default_relays = input("Use default relays? (y/n): ").lower() == 'y'
        if not use_default_relays:
            relays = input("Enter relay URLs (comma-separated): ").strip()
            credentials["nostr_relays"] = [r.strip() for r in relays.split(',') if r.strip()]
    
    print("\n--- X/Twitter Configuration ---")
    setup_twitter = input("Configure X/Twitter? (y/n): ").lower() == 'y'
    if setup_twitter:
        print("Get these from https://developer.twitter.com/en/portal/dashboard")
        credentials["twitter_consumer_key"] = input("Consumer Key (API Key): ").strip()
        credentials["twitter_consumer_secret"] = input("Consumer Secret (API Secret): ").strip()
        credentials["twitter_access_token"] = input("Access Token: ").strip()
        credentials["twitter_access_secret"] = input("Access Token Secret: ").strip()
    
    print("\n--- Threads Configuration ---")
    setup_threads = input("Configure Threads? (y/n): ").lower() == 'y'
    if setup_threads:
        print("Get access token from Meta for Developers")
        credentials["threads_access_token"] = input("Threads Access Token: ").strip()
        credentials["threads_user_id"] = input("Threads User ID: ").strip()
    
    # Save to keyring using secret-tool
    json_data = json.dumps(credentials)
    
    try:
        subprocess.run([
            "secret-tool", "store",
            "--label=Doh! Credentials",
            "application", "com.sgtapple.doh",
            "type", "credentials"
        ], input=json_data.encode(), check=True)
        
        print("\n✓ Credentials saved successfully!")
        print("\nYou can now run: just run")
        
    except subprocess.CalledProcessError:
        print("\n✗ Failed to save credentials.")
        print("Make sure 'secret-tool' is installed (part of libsecret-tools)")
        print("\nOn Ubuntu/Debian: sudo apt install libsecret-tools")
        sys.exit(1)
    except FileNotFoundError:
        print("\n✗ 'secret-tool' not found.")
        print("Install it with: sudo apt install libsecret-tools")
        sys.exit(1)

if __name__ == "__main__":
    setup_credentials()
