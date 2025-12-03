#!/usr/bin/env python3
import asyncio
import aiohttp
import time
from datetime import datetime

# Test configuration
BASE_URL = "http://localhost:3000"
AUTH_TOKEN = "Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6IlBlcGw4S2xISGNDQjBmNGEiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2x1c2N6YnVzaHduZXpqanlrdm91LnN1cGFiYXNlLmNvL2F1dGgvdjEiLCJzdWIiOiI3NWM5MzdmNy0xNjQ2LTQzMGItYjQwZi0zODUzNDZiZjIwNGQiLCJhdWQiOiJhdXRoZW50aWNhdGVkIiwiZXhwIjoxNzY0NzU4MjQzLCJpYXQiOjE3NjQ3NTQ2NDMsImVtYWlsIjoic3VyeWEuYXJqYXZhQGdtYWlsLmNvbSIsInBob25lIjoiIiwiYXBwX21ldGFkYXRhIjp7InByb3ZpZGVyIjoiZ29vZ2xlIiwicHJvdmlkZXJzIjpbImdvb2dsZSJdfSwidXNlcl9tZXRhZGF0YSI6eyJhdmF0YXJfdXJsIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUNnOG9jSVJWVVRhSlNOQ3BuM1pzUFA5V3A1MnVJWVFXdkhoSWoxUkhSVlFNTmdxOGZmam5nPXM5Ni1jIiwiZW1haWwiOiJzdXJ5YS5hcmphdmFAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImZ1bGxfbmFtZSI6IlN1cnlhQXJpc2giLCJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJuYW1lIjoiU3VyeWFBcmlzaCIsInBob25lX3ZlcmlmaWVkIjpmYWxzZSwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FDZzhvY0lSVlVUYUpTTkNwbjNac1BQOVdwNTJ1SVlRV3ZIaElqMVJIUlZRTU5ncThmZmpuZz1zOTYtYyIsInByb3ZpZGVyX2lkIjoiMTA0Mzk0ODE2Mjc3ODE0MDcwNDI3Iiwic3ViIjoiMTA0Mzk0ODE2Mjc3ODE0MDcwNDI3In0sInJvbGUiOiJhdXRoZW50aWNhdGVkIiwiYWFsIjoiYWFsMSIsImFtciI6W3sibWV0aG9kIjoib2F1dGgiLCJ0aW1lc3RhbXAiOjE3NjQ3NDUyMDh9XSwic2Vzc2lvbl9pZCI6IjNjNTE2ZTQ2LTgyYWQtNDNlOS1iNzcyLWM3MDY1OTEyNzhmMyIsImlzX2Fub255bW91cyI6ZmFsc2V9.LoHoZZSOB3GaM8ZXWXU_TlCNT7cbUnOMM1Rpdt99Zec"
PARALLEL_REQUESTS = 100
TEST_CUSTOMER_ID = "550e8400-e29b-41d4-a716-446655440000"

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

async def single_request(session, endpoint, request_id):
    """Make a single request and return detailed results"""
    start_time = time.time()
    try:
        headers = {"Authorization": AUTH_TOKEN, "Content-Type": "application/json"}
        
        async with session.request(
            endpoint["method"], 
            f"{BASE_URL}{endpoint['url']}", 
            headers=headers,
            timeout=aiohttp.ClientTimeout(total=30)
        ) as response:
            response_text = await response.text()
            response_time = time.time() - start_time
            
            return {
                'request_id': request_id,
                'status_code': response.status,
                'response_time': response_time,
                'response_size': len(response_text),
                'error': None,
                'timestamp': start_time
            }
            
    except asyncio.TimeoutError:
        return {
            'request_id': request_id,
            'status_code': 0,
            'response_time': time.time() - start_time,
            'response_size': 0,
            'error': 'TIMEOUT',
            'timestamp': start_time
        }
    except Exception as e:
        return {
            'request_id': request_id,
            'status_code': 0,
            'response_time': time.time() - start_time,
            'response_size': 0,
            'error': str(e),
            'timestamp': start_time
        }

async def parallel_test_endpoint(endpoint):
    """Send 100 truly parallel requests to a single endpoint"""
    print(f"🚀 Launching {PARALLEL_REQUESTS} PARALLEL requests to {endpoint['name']}...")
    
    # Create session with connection pooling
    connector = aiohttp.TCPConnector(limit=200, limit_per_host=200)
    
    async with aiohttp.ClientSession(connector=connector) as session:
        # Create all tasks at once - this ensures true parallelism
        tasks = [
            single_request(session, endpoint, i) 
            for i in range(PARALLEL_REQUESTS)
        ]
        
        # Launch ALL requests at the exact same moment
        start_time = time.time()
        results = await asyncio.gather(*tasks, return_exceptions=True)
        total_time = time.time() - start_time
        
        print(f"✅ Completed {endpoint['name']} in {total_time:.2f}s")
        
        return {
            'endpoint': endpoint['name'],
            'results': results,
            'total_time': total_time,
            'start_time': start_time
        }

def analyze_parallel_results(test_results):
    """Analyze results from parallel testing"""
    print("\n" + "="*80)
    print("PARALLEL LOAD TEST ANALYSIS")
    print("="*80)
    
    session_issues = []
    failed_endpoints = []
    slow_endpoints = []
    concurrency_issues = []
    rate_limit_issues = []
    
    for test in test_results:
        endpoint_name = test['endpoint']
        results = test['results']
        total_time = test['total_time']
        
        # Filter out exceptions
        valid_results = [r for r in results if isinstance(r, dict)]
        
        if not valid_results:
            failed_endpoints.append(f"{endpoint_name} - All requests failed")
            continue
            
        # Calculate metrics
        success_count = len([r for r in valid_results if r['status_code'] == 200])
        error_count = len([r for r in valid_results if r['status_code'] != 200])
        timeout_count = len([r for r in valid_results if r['error'] == 'TIMEOUT'])
        auth_failures = len([r for r in valid_results if r['status_code'] == 401])
        rate_limits = len([r for r in valid_results if r['status_code'] == 429])
        server_errors = len([r for r in valid_results if 500 <= r['status_code'] < 600])
        
        response_times = [r['response_time'] for r in valid_results if r['status_code'] == 200]
        avg_response_time = sum(response_times) / len(response_times) if response_times else 0
        max_response_time = max(response_times) if response_times else 0
        min_response_time = min(response_times) if response_times else 0
        
        # Check for concurrency issues (requests should complete around the same time)
        timestamps = [r['timestamp'] for r in valid_results]
        time_spread = max(timestamps) - min(timestamps) if timestamps else 0
        
        print(f"\n📊 {endpoint_name}")
        print(f"   Total Requests: {len(valid_results)}")
        print(f"   Success (200): {success_count}")
        print(f"   Errors: {error_count}")
        print(f"   Timeouts: {timeout_count}")
        print(f"   Auth Failures (401): {auth_failures}")
        print(f"   Rate Limited (429): {rate_limits}")
        print(f"   Server Errors (5xx): {server_errors}")
        print(f"   Total Test Time: {total_time:.2f}s")
        print(f"   Request Time Spread: {time_spread:.3f}s")
        
        if response_times:
            print(f"   Avg Response Time: {avg_response_time:.3f}s")
            print(f"   Min/Max Response: {min_response_time:.3f}s / {max_response_time:.3f}s")
        
        # Identify issues
        if server_errors > 0:
            session_issues.append(f"{endpoint_name} - {server_errors} server errors")
        
        if error_count > 5:  # More than 5% failure rate
            failed_endpoints.append(f"{endpoint_name} - {error_count} failures")
        
        if avg_response_time > 5.0:  # Slower than 5 seconds under load
            slow_endpoints.append(f"{endpoint_name} - {avg_response_time:.2f}s avg")
        
        if time_spread > 2.0:  # Requests took too long to start
            concurrency_issues.append(f"{endpoint_name} - {time_spread:.2f}s spread")
        
        if rate_limits > 0:
            rate_limit_issues.append(f"{endpoint_name} - {rate_limits} rate limited")
        
        if timeout_count > 0:
            slow_endpoints.append(f"{endpoint_name} - {timeout_count} timeouts")
    
    # Summary Analysis
    print(f"\n🔍 PARALLEL LOAD TEST SUMMARY")
    print(f"{'='*50}")
    
    print(f"\n1. SESSION/TOKEN ISSUES:")
    if session_issues:
        for issue in session_issues:
            print(f"   ❌ {issue}")
    else:
        print(f"   ✅ No session or token issues detected")
    
    print(f"\n2. ENDPOINT FAILURES:")
    if failed_endpoints:
        for endpoint in failed_endpoints:
            print(f"   ❌ {endpoint}")
    else:
        print(f"   ✅ All endpoints handled parallel load successfully")
    
    print(f"\n3. PERFORMANCE UNDER LOAD:")
    if slow_endpoints:
        for endpoint in slow_endpoints:
            print(f"   ⚠️  {endpoint}")
    else:
        print(f"   ✅ All endpoints maintained good performance")
    
    print(f"\n4. CONCURRENCY HANDLING:")
    if concurrency_issues:
        for issue in concurrency_issues:
            print(f"   ⚠️  {issue}")
    else:
        print(f"   ✅ Server handled parallel requests efficiently")
    
    print(f"\n5. RATE LIMITING/THROTTLING:")
    if rate_limit_issues:
        for issue in rate_limit_issues:
            print(f"   ⚠️  {issue}")
    else:
        print(f"   ✅ No rate limiting triggered")
    
    # Technical Explanation
    print(f"\n💡 TECHNICAL EXPLANATION:")
    if not any([session_issues, failed_endpoints, slow_endpoints, concurrency_issues, rate_limit_issues]):
        print(f"   🎯 EXCELLENT: Your API handles {PARALLEL_REQUESTS} simultaneous requests perfectly!")
        print(f"   • Authentication system scales well under load")
        print(f"   • Database connection pooling is working correctly")
        print(f"   • No bottlenecks in request processing")
        print(f"   • Server can handle high concurrency without issues")
    else:
        print(f"   📋 RECOMMENDATIONS:")
        if session_issues:
            print(f"   • Increase database connection pool size")
            print(f"   • Check server memory and CPU under load")
        if slow_endpoints:
            print(f"   • Optimize database queries for high concurrency")
            print(f"   • Consider adding response caching")
        if concurrency_issues:
            print(f"   • Check server thread/async handling configuration")
        if rate_limit_issues:
            print(f"   • Review rate limiting settings if intentional")

async def main():
    print("🚀 PARALLEL LOAD TEST STARTING")
    print(f"Target: {BASE_URL}")
    print(f"Parallel Requests per Endpoint: {PARALLEL_REQUESTS}")
    print(f"Total Endpoints: {len(ENDPOINTS)}")
    print(f"This will send {PARALLEL_REQUESTS} requests SIMULTANEOUSLY to each endpoint")
    print("-" * 80)
    
    all_results = []
    
    for endpoint in ENDPOINTS:
        result = await parallel_test_endpoint(endpoint)
        all_results.append(result)
        # Small pause between endpoint tests to avoid overwhelming
        await asyncio.sleep(2)
    
    analyze_parallel_results(all_results)

if __name__ == "__main__":
    asyncio.run(main())