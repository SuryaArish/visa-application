#!/usr/bin/env python3
import asyncio
import aiohttp
import time
import json
from datetime import datetime

# Test configuration
BASE_URL = "http://localhost:3000"
AUTH_TOKEN = "Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6IlBlcGw4S2xISGNDQjBmNGEiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2x1c2N6YnVzaHduZXpqanlrdm91LnN1cGFiYXNlLmNvL2F1dGgvdjEiLCJzdWIiOiI3NWM5MzdmNy0xNjQ2LTQzMGItYjQwZi0zODUzNDZiZjIwNGQiLCJhdWQiOiJhdXRoZW50aWNhdGVkIiwiZXhwIjoxNzY0NzU4MjQzLCJpYXQiOjE3NjQ3NTQ2NDMsImVtYWlsIjoic3VyeWEuYXJqYXZhQGdtYWlsLmNvbSIsInBob25lIjoiIiwiYXBwX21ldGFkYXRhIjp7InByb3ZpZGVyIjoiZ29vZ2xlIiwicHJvdmlkZXJzIjpbImdvb2dsZSJdfSwidXNlcl9tZXRhZGF0YSI6eyJhdmF0YXJfdXJsIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUNnOG9jSVJWVVRhSlNOQ3BuM1pzUFA5V3A1MnVJWVFXdkhoSWoxUkhSVlFNTmdxOGZmam5nPXM5Ni1jIiwiZW1haWwiOiJzdXJ5YS5hcmphdmFAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImZ1bGxfbmFtZSI6IlN1cnlhQXJpc2giLCJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJuYW1lIjoiU3VyeWFBcmlzaCIsInBob25lX3ZlcmlmaWVkIjpmYWxzZSwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FDZzhvY0lSVlVUYUpTTkNwbjNac1BQOVdwNTJ1SVlRV3ZIaElqMVJIUlZRTU5ncThmZmpuZz1zOTYtYyIsInByb3ZpZGVyX2lkIjoiMTA0Mzk0ODE2Mjc3ODE0MDcwNDI3Iiwic3ViIjoiMTA0Mzk0ODE2Mjc3ODE0MDcwNDI3In0sInJvbGUiOiJhdXRoZW50aWNhdGVkIiwiYWFsIjoiYWFsMSIsImFtciI6W3sibWV0aG9kIjoib2F1dGgiLCJ0aW1lc3RhbXAiOjE3NjQ3NDUyMDh9XSwic2Vzc2lvbl9pZCI6IjNjNTE2ZTQ2LTgyYWQtNDNlOS1iNzcyLWM3MDY1OTEyNzhmMyIsImlzX2Fub255bW91cyI6ZmFsc2V9.LoHoZZSOB3GaM8ZXWXU_TlCNT7cbUnOMM1Rpdt99Zec"
CONCURRENT_REQUESTS = 50
TEST_CUSTOMER_ID = "550e8400-e29b-41d4-a716-446655440000"  # Sample UUID

# API endpoints to test
ENDPOINTS = [
    {"method": "GET", "url": "/health", "name": "Health Check"},
    {"method": "GET", "url": "/hello", "name": "Test Connection"},
    {"method": "GET", "url": "/customers", "name": "Get All Customers"},
    {"method": "GET", "url": "/h1b_customer/all", "name": "Get All Customers No Filter"},
    {"method": "GET", "url": f"/get_customer_by_id/{TEST_CUSTOMER_ID}", "name": "Get Customer By ID"},
    {"method": "GET", "url": "/get_customer_by_email/test@example.com", "name": "Get Customer By Email"},
    {"method": "GET", "url": "/h1b_customer/by_login_email/test@example.com", "name": "Get Customer By Login Email"},
]

class LoadTestResults:
    def __init__(self):
        self.results = {}
        
    def add_result(self, endpoint_name, status_code, response_time, error=None):
        if endpoint_name not in self.results:
            self.results[endpoint_name] = {
                'success_count': 0,
                'error_count': 0,
                'total_time': 0,
                'min_time': float('inf'),
                'max_time': 0,
                'status_codes': {},
                'errors': []
            }
        
        result = self.results[endpoint_name]
        
        if error:
            result['error_count'] += 1
            result['errors'].append(str(error))
        else:
            result['success_count'] += 1
            result['total_time'] += response_time
            result['min_time'] = min(result['min_time'], response_time)
            result['max_time'] = max(result['max_time'], response_time)
        
        if status_code in result['status_codes']:
            result['status_codes'][status_code] += 1
        else:
            result['status_codes'][status_code] = 1

async def make_request(session, endpoint, semaphore):
    async with semaphore:
        start_time = time.time()
        try:
            headers = {"Authorization": AUTH_TOKEN, "Content-Type": "application/json"}
            
            async with session.request(
                endpoint["method"], 
                f"{BASE_URL}{endpoint['url']}", 
                headers=headers,
                timeout=aiohttp.ClientTimeout(total=30)
            ) as response:
                await response.text()  # Read response body
                response_time = time.time() - start_time
                return endpoint["name"], response.status, response_time, None
                
        except Exception as e:
            response_time = time.time() - start_time
            return endpoint["name"], 0, response_time, e

async def load_test_endpoint(endpoint, num_requests=CONCURRENT_REQUESTS):
    semaphore = asyncio.Semaphore(10)  # Limit concurrent requests
    results = LoadTestResults()
    
    print(f"Testing {endpoint['name']} with {num_requests} requests...")
    
    async with aiohttp.ClientSession() as session:
        tasks = []
        for _ in range(num_requests):
            task = make_request(session, endpoint, semaphore)
            tasks.append(task)
        
        responses = await asyncio.gather(*tasks)
        
        for name, status_code, response_time, error in responses:
            results.add_result(name, status_code, response_time, error)
    
    return results

def analyze_results(all_results):
    print("\n" + "="*80)
    print("LOAD TEST ANALYSIS REPORT")
    print("="*80)
    
    session_issues = []
    slow_endpoints = []
    error_endpoints = []
    auth_failures = []
    
    for endpoint_name, result in all_results.items():
        total_requests = result['success_count'] + result['error_count']
        success_rate = (result['success_count'] / total_requests) * 100 if total_requests > 0 else 0
        avg_time = result['total_time'] / result['success_count'] if result['success_count'] > 0 else 0
        
        print(f"\n📊 {endpoint_name}")
        print(f"   Total Requests: {total_requests}")
        print(f"   Success Rate: {success_rate:.1f}%")
        print(f"   Average Response Time: {avg_time:.3f}s")
        
        if result['success_count'] > 0:
            print(f"   Min/Max Response Time: {result['min_time']:.3f}s / {result['max_time']:.3f}s")
        
        print(f"   Status Codes: {result['status_codes']}")
        
        if result['errors']:
            print(f"   Errors: {len(result['errors'])} unique errors")
            error_endpoints.append(endpoint_name)
        
        # Check for issues
        if avg_time > 2.0:  # Slow if > 2 seconds
            slow_endpoints.append(f"{endpoint_name} ({avg_time:.2f}s)")
        
        if 401 in result['status_codes']:
            auth_failures.append(f"{endpoint_name} ({result['status_codes'][401]} failures)")
        
        if 500 in result['status_codes'] or 502 in result['status_codes'] or 503 in result['status_codes']:
            session_issues.append(f"{endpoint_name} (Server errors: {result['status_codes']})")
    
    # Summary Analysis
    print(f"\n🔍 ANALYSIS SUMMARY")
    print(f"{'='*50}")
    
    print(f"\n1. SESSION-RELATED ISSUES:")
    if session_issues:
        for issue in session_issues:
            print(f"   ❌ {issue}")
    else:
        print(f"   ✅ No session-related issues detected")
    
    print(f"\n2. SLOW ENDPOINTS:")
    if slow_endpoints:
        for endpoint in slow_endpoints:
            print(f"   ⚠️  {endpoint}")
    else:
        print(f"   ✅ All endpoints responding within acceptable time")
    
    print(f"\n3. UNEXPECTED ERRORS:")
    if error_endpoints:
        for endpoint in error_endpoints:
            print(f"   ❌ {endpoint}")
    else:
        print(f"   ✅ No unexpected errors detected")
    
    print(f"\n4. AUTHENTICATION FAILURES:")
    if auth_failures:
        for failure in auth_failures:
            print(f"   ❌ {failure}")
    else:
        print(f"   ✅ No authentication failures detected")
    
    # Recommendations
    print(f"\n💡 RECOMMENDATIONS:")
    if slow_endpoints:
        print(f"   • Optimize database queries for slow endpoints")
        print(f"   • Consider adding caching for frequently accessed data")
    if auth_failures:
        print(f"   • Check Supabase token validity and configuration")
        print(f"   • Verify authentication middleware is working correctly")
    if session_issues:
        print(f"   • Check database connection pool settings")
        print(f"   • Monitor server resources during high load")
    if not (slow_endpoints or auth_failures or session_issues or error_endpoints):
        print(f"   ✅ API is performing well under load!")

async def main():
    print("🚀 Starting API Load Test...")
    print(f"Target: {BASE_URL}")
    print(f"Concurrent Requests per Endpoint: {CONCURRENT_REQUESTS}")
    print(f"Total Endpoints: {len(ENDPOINTS)}")
    
    all_results = {}
    
    for endpoint in ENDPOINTS:
        results = await load_test_endpoint(endpoint, CONCURRENT_REQUESTS)
        all_results.update(results.results)
        await asyncio.sleep(1)  # Brief pause between endpoint tests
    
    analyze_results(all_results)

if __name__ == "__main__":
    asyncio.run(main())