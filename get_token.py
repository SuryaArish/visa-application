#!/usr/bin/env python3
import requests
import json

# Your Supabase credentials
SUPABASE_URL = "https://lusczbushwnezjjykvou.supabase.co"
SUPABASE_API_KEY = "sb_publishable_D_Q9SuFj492ZnKzeigb5Nw_ymhH6LU0"

def get_test_token():
    """Get a test token by creating a test user or using existing session"""
    
    # Try to sign up a test user (or sign in if exists)
    auth_url = f"{SUPABASE_URL}/auth/v1/signup"
    
    test_user = {
        "email": "loadtest@example.com",
        "password": "testpassword123"
    }
    
    headers = {
        "apikey": SUPABASE_API_KEY,
        "Content-Type": "application/json"
    }
    
    try:
        # Try signup first
        response = requests.post(auth_url, json=test_user, headers=headers)
        
        if response.status_code == 400:
            # User might already exist, try signin
            signin_url = f"{SUPABASE_URL}/auth/v1/token?grant_type=password"
            response = requests.post(signin_url, json=test_user, headers=headers)
        
        if response.status_code == 200:
            data = response.json()
            token = data.get('access_token')
            if token:
                print(f"✅ Got test token: Bearer {token}")
                return f"Bearer {token}"
        
        print(f"❌ Failed to get token: {response.status_code} - {response.text}")
        return None
        
    except Exception as e:
        print(f"❌ Error getting token: {e}")
        return None

if __name__ == "__main__":
    token = get_test_token()
    if token:
        print(f"\nUse this token in load_test.py:")
        print(f'AUTH_TOKEN = "{token}"')
    else:
        print("\n⚠️  Using default token - authentication tests may fail")