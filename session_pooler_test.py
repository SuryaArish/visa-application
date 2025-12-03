#!/usr/bin/env python3
import asyncio
import aiohttp
import time
import json
from datetime import datetime

# Test configuration
BASE_URL = "http://localhost:3000"
AUTH_TOKEN = "Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6IlBlcGw4S2xISGNDQjBmNGEiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2x1c2N6YnVzaHduZXpqanlrdm91LnN1cGFiYXNlLmNvL2F1dGgvdjEiLCJzdWIiOiI3NWM5MzdmNy0xNjQ2LTQzMGItYjQwZi0zODUzNDZiZjIwNGQiLCJhdWQiOiJhdXRoZW50aWNhdGVkIiwiZXhwIjoxNzY0NzU4MjQzLCJpYXQiOjE3NjQ3NTQ2NDMsImVtYWlsIjoic3VyeWEuYXJqYXZhQGdtYWlsLmNvbSIsInBob25lIjoiIiwiYXBwX21ldGFkYXRhIjp7InByb3ZpZGVyIjoiZ29vZ2xlIiwicHJvdmlkZXJzIjpbImdvb2dsZSJdfSwidXNlcl9tZXRhZGF0YSI6eyJhdmF0YXJfdXJsIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUNnOG9jSVJWVVRhSlNOQ3BuM1pzUFA5V3A1MnVJWVFXdkhoSWoxUkhSVlFNTmdxOGZmam5nPXM5Ni1jIiwiZW1haWwiOiJzdXJ5YS5hcmphdmFAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImZ1bGxfbmFtZSI6IlN1cnlhQXJpc2giLCJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJuYW1lIjoiU3VyeWFBcmlzaCIsInBob25lX3ZlcmlmaWVkIjpmYWxzZSwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hL0FDZzhvY0lSVlVUYUpTTkNwbjNac1BQOVdwNTJ1SVlRV3ZIaElqMVJIUlZRTU5ncThmZmpuZz1zOTYtYyIsInByb3ZpZGVyX2lkIjoiMTA0Mzk0ODE2Mjc3ODE0MDcwNDI3Iiwic3ViIjoiMTA0Mzk0ODE2Mjc3ODE0MDcwNDI3In0sInJvbGUiOiJhdXRoZW50aWNhdGVkIiwiYWFsIjoiYWFsMSIsImFtciI6W3sibWV0aG9kIjoib2F1dGgiLCJ0aW1lc3RhbXAiOjE3NjQ3NDUyMDh9XSwic2Vzc2lvbl9pZCI6IjNjNTE2ZTQ2LTgyYWQtNDNlOS1iNzcyLWM3MDY1OTEyNzhmMyIsImlzX2Fub255bW91cyI6ZmFsc2V9.LoHoZZSOB3GaM8ZXWXU_TlCNT7cbUnOMM1Rpdt99Zec"
PARALLEL_REQUESTS = 100
TEST_CUSTOMER_ID = "550e8400-e29b-41d4-a716-446655440000"

# Focus on database-heavy endpoints that would trigger session pooler issues
ENDPOINTS = [
    {"method": "GET", "url": "/customers", "name": "Get All Customers", "db_heavy": True},
    {"method": "GET", "url": "/h1b_customer/all", "name": "Get All Customers No Filter", "db_heavy": True},
    {"method": "GET", "url": f"/get_customer_by_id/{TEST_CUSTOMER_ID}", "name": "Get Customer By ID", "db_heavy": True},
    {"method": "GET", "url": "/get_customer_by_email/test@example.com", "name": "Get Customer By Email", "db_heavy": True},
    {"method": "GET", "url": "/h1b_customer/by_login_email/test@example.com", "name": "Get Customer By Login Email", "db_heavy": True},
    {"method": "GET", "url": "/hello", "name": "Test Connection", "db_heavy": True},
    {"method": "GET", "url": "/health", "name": "Health Check", "db_heavy": False},
]

async def detailed_request(session, endpoint, request_id):
    """Make a request with detailed error tracking for session pooler issues"""
    start_time = time.time()
    try:
        headers = {"Authorization": AUTH_TOKEN, "Content-Type": "application/json"}
        
        async with session.request(
            endpoint["method"], 
            f"{BASE_URL}{endpoint['url']}", 
            headers=headers,
            timeout=aiohttp.ClientTimeout(total=60)  # Longer timeout to catch slow responses
        ) as response:
            response_text = await response.text()
            response_time = time.time() - start_time
            
            # Check for specific error patterns
            error_type = None
            if response.status == 500:
                if "pool" in response_text.lower() or "connection" in response_text.lower():
                    error_type = "SESSION_POOLER_ERROR"
                elif "timeout" in response_text.lower():
                    error_type = "DATABASE_TIMEOUT"
                else:
                    error_type = "SERVER_ERROR"
            
            return {
                'request_id': request_id,
                'status_code': response.status,
                'response_time': response_time,
                'response_size': len(response_text),
                'error_type': error_type,
                'response_body': response_text[:200] if response.status != 200 else "",
                'timestamp': start_time
            }
            
    except asyncio.TimeoutError:
        return {
            'request_id': request_id,
            'status_code': 0,
            'response_time': time.time() - start_time,
            'response_size': 0,
            'error_type': 'CLIENT_TIMEOUT',
            'response_body': "",
            'timestamp': start_time
        }
    except aiohttp.ClientConnectorError as e:
        return {
            'request_id': request_id,
            'status_code': 0,
            'response_time': time.time() - start_time,
            'response_size': 0,
            'error_type': 'CONNECTION_ERROR',
            'response_body': str(e)[:200],
            'timestamp': start_time
        }
    except Exception as e:
        return {
            'request_id': request_id,
            'status_code': 0,
            'response_time': time.time() - start_time,
            'response_size': 0,
            'error_type': 'UNKNOWN_ERROR',
            'response_body': str(e)[:200],
            'timestamp': start_time
        }

async def stress_test_endpoint(endpoint):
    """Stress test with maximum concurrency to trigger session pooler issues"""
    print(f"🔥 STRESS TESTING {endpoint['name']} with {PARALLEL_REQUESTS} parallel requests...")
    
    # Use maximum connections to stress the system
    connector = aiohttp.TCPConnector(
        limit=300,           # Total connection pool
        limit_per_host=300,  # Per-host limit
        ttl_dns_cache=300,
        use_dns_cache=True,
    )
    
    async with aiohttp.ClientSession(connector=connector) as session:
        # Create all tasks simultaneously
        tasks = [
            detailed_request(session, endpoint, i) 
            for i in range(PARALLEL_REQUESTS)
        ]
        
        # Launch ALL requests at exactly the same moment
        print(f"   🚀 Launching {PARALLEL_REQUESTS} requests simultaneously...")
        start_time = time.time()
        results = await asyncio.gather(*tasks, return_exceptions=True)
        total_time = time.time() - start_time
        
        print(f"   ✅ Completed in {total_time:.2f}s")
        
        return {
            'endpoint': endpoint['name'],
            'db_heavy': endpoint['db_heavy'],
            'results': [r for r in results if isinstance(r, dict)],
            'total_time': total_time,
            'start_time': start_time
        }

def analyze_session_pooler_issues(test_results):
    """Detailed analysis focusing on session pooler and connection issues"""
    print("\n" + "="*80)
    print("SESSION POOLER & CONNECTION ANALYSIS")
    print("="*80)
    
    session_pooler_errors = []
    connection_issues = []
    timeout_issues = []
    server_errors = []
    performance_degradation = []
    
    total_requests = 0
    total_500_errors = 0
    total_timeouts = 0
    total_connection_errors = 0
    
    for test in test_results:
        endpoint_name = test['endpoint']
        is_db_heavy = test['db_heavy']
        results = test['results']
        total_time = test['total_time']
        
        if not results:
            continue
            
        total_requests += len(results)
        
        # Categorize errors
        status_200 = len([r for r in results if r['status_code'] == 200])
        status_500 = len([r for r in results if r['status_code'] == 500])
        timeouts = len([r for r in results if r['error_type'] == 'CLIENT_TIMEOUT'])
        conn_errors = len([r for r in results if r['error_type'] == 'CONNECTION_ERROR'])
        pooler_errors = len([r for r in results if r['error_type'] == 'SESSION_POOLER_ERROR'])
        
        total_500_errors += status_500
        total_timeouts += timeouts
        total_connection_errors += conn_errors
        
        # Response time analysis
        success_times = [r['response_time'] for r in results if r['status_code'] == 200]
        avg_time = sum(success_times) / len(success_times) if success_times else 0
        max_time = max(success_times) if success_times else 0
        
        print(f"\n📊 {endpoint_name} {'(DB Heavy)' if is_db_heavy else '(Light)'}")
        print(f"   Total Requests: {len(results)}")
        print(f"   Success (200): {status_200}")
        print(f"   Server Errors (500): {status_500}")
        print(f"   Timeouts: {timeouts}")
        print(f"   Connection Errors: {conn_errors}")
        print(f"   Session Pooler Errors: {pooler_errors}")
        print(f"   Test Duration: {total_time:.2f}s")
        
        if success_times:
            print(f"   Avg Response Time: {avg_time:.3f}s")
            print(f"   Max Response Time: {max_time:.3f}s")
        
        # Sample error messages
        error_samples = [r['response_body'] for r in results if r['status_code'] == 500][:3]
        if error_samples:
            print(f"   Sample Errors:")
            for i, error in enumerate(error_samples, 1):
                print(f"     {i}. {error}")
        
        # Identify specific issues
        if pooler_errors > 0:
            session_pooler_errors.append(f"{endpoint_name}: {pooler_errors} pooler errors")
        
        if conn_errors > 0:
            connection_issues.append(f"{endpoint_name}: {conn_errors} connection errors")
        
        if timeouts > 0:
            timeout_issues.append(f"{endpoint_name}: {timeouts} timeouts")
        
        if status_500 > 0:
            server_errors.append(f"{endpoint_name}: {status_500} server errors")
        
        if is_db_heavy and avg_time > 10.0:
            performance_degradation.append(f"{endpoint_name}: {avg_time:.2f}s avg (degraded)")
    
    # Overall Analysis
    print(f"\n🔍 ROOT CAUSE ANALYSIS")
    print(f"{'='*50}")
    
    success_rate = ((total_requests - total_500_errors - total_timeouts - total_connection_errors) / total_requests * 100) if total_requests > 0 else 0
    
    print(f"\n📈 OVERALL STATISTICS:")
    print(f"   Total Requests: {total_requests}")
    print(f"   Success Rate: {success_rate:.1f}%")
    print(f"   500 Errors: {total_500_errors}")
    print(f"   Timeouts: {total_timeouts}")
    print(f"   Connection Errors: {total_connection_errors}")
    
    print(f"\n🔍 SESSION POOLER BEHAVIOR:")
    if session_pooler_errors:
        print(f"   ❌ SESSION POOLER TIMEOUT DETECTED!")
        for error in session_pooler_errors:
            print(f"      • {error}")
        print(f"   🔧 ROOT CAUSE: Database connection pool exhausted")
    else:
        print(f"   ✅ No explicit session pooler errors detected")
    
    print(f"\n🔍 CONNECTION LIMITS:")
    if connection_issues or total_connection_errors > 0:
        print(f"   ❌ CONNECTION LIMIT ISSUES DETECTED!")
        for issue in connection_issues:
            print(f"      • {issue}")
        print(f"   🔧 LIKELY CAUSE: Server connection limit reached")
    else:
        print(f"   ✅ No connection limit issues")
    
    print(f"\n🔍 TIMEOUT ANALYSIS:")
    if timeout_issues or total_timeouts > 0:
        print(f"   ⚠️  TIMEOUT ISSUES DETECTED!")
        for issue in timeout_issues:
            print(f"      • {issue}")
        print(f"   🔧 POSSIBLE CAUSES: Slow queries, connection pool exhaustion, CPU bottleneck")
    else:
        print(f"   ✅ No timeout issues")
    
    print(f"\n🔍 SERVER ERROR ANALYSIS:")
    if server_errors or total_500_errors > 0:
        print(f"   ❌ SERVER ERRORS DETECTED!")
        for error in server_errors:
            print(f"      • {error}")
        print(f"   🔧 INVESTIGATION NEEDED: Check server logs for specific error details")
    else:
        print(f"   ✅ No server errors")
    
    # Technical Diagnosis
    print(f"\n💡 TECHNICAL DIAGNOSIS:")
    
    if total_500_errors > 20 or total_timeouts > 20:
        print(f"   🚨 CRITICAL ISSUES FOUND:")
        print(f"   • Database connection pool is likely set to 15 connections")
        print(f"   • {PARALLEL_REQUESTS} parallel requests exceed connection limit")
        print(f"   • Session pooler timeout occurs when connections are exhausted")
        print(f"   • API is properly async but database is the bottleneck")
        
        print(f"\n   🔧 RECOMMENDED FIXES:")
        print(f"   1. Increase database connection pool size (current likely: 15)")
        print(f"   2. Add connection pooling configuration in Rust code")
        print(f"   3. Implement query optimization for high concurrency")
        print(f"   4. Consider connection retry logic")
        
    elif success_rate > 95:
        print(f"   ✅ SYSTEM HANDLING LOAD WELL:")
        print(f"   • Connection pooling is adequate for current load")
        print(f"   • No session pooler timeouts detected")
        print(f"   • API concurrency handling is working correctly")
        
    else:
        print(f"   ⚠️  MODERATE ISSUES:")
        print(f"   • Some connection pressure detected")
        print(f"   • May need connection pool tuning")
        print(f"   • Monitor under sustained load")

async def main():
    print("🔥 SESSION POOLER STRESS TEST")
    print(f"Target: {BASE_URL}")
    print(f"Parallel Requests per Endpoint: {PARALLEL_REQUESTS}")
    print(f"Focus: Detecting session pooler timeouts and connection limits")
    print("-" * 80)
    
    all_results = []
    
    for endpoint in ENDPOINTS:
        result = await stress_test_endpoint(endpoint)
        all_results.append(result)
        # Brief pause to let system recover
        await asyncio.sleep(3)
    
    analyze_session_pooler_issues(all_results)

if __name__ == "__main__":
    asyncio.run(main())