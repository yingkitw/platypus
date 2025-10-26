# Staging Deployment & Monitoring Setup

**Date**: October 26, 2025  
**Status**: Ready for Implementation  
**Timeline**: 2 weeks

## Phase 1: Staging Environment Setup (Days 1-3)

### 1.1 Infrastructure Provisioning

#### Server Configuration
```bash
# Staging server specs
- CPU: 4 cores (vs 8 for production)
- RAM: 8GB (vs 16GB for production)
- Storage: 100GB SSD (vs 500GB for production)
- Network: 100 Mbps (vs 1 Gbps for production)
- OS: Ubuntu 22.04 LTS
```

#### Database Setup
```bash
# PostgreSQL staging database
- Version: 15.x
- Backup: Daily
- Replication: None (staging only)
- Monitoring: Enabled
- Logging: Verbose
```

#### Load Balancer Configuration
```nginx
upstream staging_backend {
    server staging-1.internal:8000;
    server staging-2.internal:8000;
    keepalive 32;
}

server {
    listen 443 ssl http2;
    server_name staging.platypus.example.com;
    
    ssl_certificate /etc/ssl/certs/staging.pem;
    ssl_certificate_key /etc/ssl/private/staging.key;
    
    location / {
        proxy_pass http://staging_backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 1.2 Application Deployment

#### Backend Deployment
```bash
#!/bin/bash
# Deploy backend to staging

# 1. Build release
cd /app/platypus
cargo build --release

# 2. Stop current service
systemctl stop platypus-staging

# 3. Deploy new version
cp target/release/platypus-cli /app/staging/
cp -r crates/platypus-proto/proto /app/staging/

# 4. Update configuration
cp config/staging.toml /app/staging/config.toml

# 5. Start service
systemctl start platypus-staging

# 6. Verify
sleep 5
curl -s http://localhost:8000/health | jq .
```

#### Frontend Deployment
```bash
#!/bin/bash
# Deploy frontend to staging

# 1. Build frontend
cd /app/platypus/frontend
npm run build

# 2. Deploy to staging S3
aws s3 sync dist/ s3://platypus-staging-frontend/ \
  --delete \
  --cache-control "max-age=3600"

# 3. Invalidate CloudFront
aws cloudfront create-invalidation \
  --distribution-id E123STAGING \
  --paths "/*"

# 4. Verify
curl -I https://staging.platypus.example.com/
```

### 1.3 Configuration Management

#### Environment Variables
```bash
# .env.staging
NODE_ENV=staging
REACT_APP_API_URL=https://staging-api.platypus.example.com
REACT_APP_WS_URL=wss://staging-api.platypus.example.com/ws
LOG_LEVEL=debug
ENABLE_PROFILING=true
ENABLE_MONITORING=true
```

#### Database Migrations
```bash
# Run migrations on staging
./scripts/migrate-db.sh staging

# Verify migrations
psql -h staging-db.internal -U platypus -d platypus_staging -c "\dt"
```

---

## Phase 2: Monitoring Setup (Days 4-7)

### 2.1 Metrics Collection

#### Prometheus Configuration
```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'platypus-staging'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: '/metrics'
    scrape_interval: 10s
    scrape_timeout: 5s

  - job_name: 'node-exporter'
    static_configs:
      - targets: ['staging-1.internal:9100', 'staging-2.internal:9100']

  - job_name: 'postgres-exporter'
    static_configs:
      - targets: ['staging-db.internal:9187']
```

#### Metrics to Monitor
```
Backend Metrics:
- http_requests_total
- http_request_duration_seconds
- http_requests_in_progress
- database_query_duration_seconds
- websocket_connections_active
- websocket_messages_total
- cache_hits_total
- cache_misses_total
- errors_total
- memory_usage_bytes
- cpu_usage_percent

Frontend Metrics:
- page_load_time_ms
- first_contentful_paint_ms
- largest_contentful_paint_ms
- cumulative_layout_shift
- first_input_delay_ms
- javascript_errors_total
- api_request_duration_ms
- cache_hit_rate_percent
```

### 2.2 Logging Setup

#### Centralized Logging
```bash
# ELK Stack Configuration
- Elasticsearch: 3 nodes, 10GB each
- Logstash: 2 instances
- Kibana: 1 instance
- Retention: 30 days
```

#### Log Levels
```
Backend:
- ERROR: All errors
- WARN: Warnings and errors
- INFO: Requests, events
- DEBUG: Detailed execution

Frontend:
- ERROR: JavaScript errors
- WARN: Warnings
- INFO: User actions
- DEBUG: Component lifecycle
```

#### Log Aggregation
```bash
# Filebeat configuration
filebeat.inputs:
- type: log
  enabled: true
  paths:
    - /var/log/platypus/*.log
  fields:
    service: platypus
    environment: staging

output.elasticsearch:
  hosts: ["elasticsearch.internal:9200"]
  index: "platypus-staging-%{+yyyy.MM.dd}"
```

### 2.3 Alerting Setup

#### Alert Rules
```yaml
# alert-rules.yml
groups:
  - name: platypus_staging
    rules:
      # High error rate
      - alert: HighErrorRate
        expr: rate(errors_total[5m]) > 0.01
        for: 5m
        annotations:
          summary: "High error rate detected"
          
      # High latency
      - alert: HighLatency
        expr: histogram_quantile(0.95, http_request_duration_seconds) > 0.5
        for: 5m
        annotations:
          summary: "High request latency"
          
      # High memory usage
      - alert: HighMemoryUsage
        expr: memory_usage_bytes > 7000000000
        for: 5m
        annotations:
          summary: "High memory usage"
          
      # Database connection pool exhausted
      - alert: DatabasePoolExhausted
        expr: database_connections_active >= database_connections_max
        for: 1m
        annotations:
          summary: "Database connection pool exhausted"
          
      # Service down
      - alert: ServiceDown
        expr: up{job="platypus-staging"} == 0
        for: 1m
        annotations:
          summary: "Chatapp service is down"
```

#### Notification Channels
```
- Slack: #platypus-staging-alerts
- Email: devops@example.com
- PagerDuty: Chatapp Staging
- SMS: Critical alerts only
```

### 2.4 Dashboards

#### Grafana Dashboards
```
1. Overview Dashboard
   - Service health
   - Request rate
   - Error rate
   - Response time
   - Active users

2. Performance Dashboard
   - CPU usage
   - Memory usage
   - Disk I/O
   - Network I/O
   - Database performance

3. Error Dashboard
   - Error rate
   - Error types
   - Error locations
   - Error trends
   - Recent errors

4. Business Dashboard
   - Active users
   - Requests per user
   - Feature usage
   - User retention
   - Revenue metrics
```

---

## Phase 3: User Testing (Days 8-14)

### 3.1 Test Plan

#### Functional Testing
```
1. User Registration & Login
   - Create account
   - Login with credentials
   - Password reset
   - Session management

2. Core Features
   - Create application
   - Add elements
   - Interact with widgets
   - View charts
   - Navigate pages

3. Data Management
   - Upload files
   - Download reports
   - Export data
   - Import data

4. Settings & Profile
   - Update profile
   - Change password
   - Configure preferences
   - Manage API keys
```

#### Performance Testing
```
1. Load Time
   - Initial page load: <2s
   - Route navigation: <500ms
   - Data loading: <1s

2. Responsiveness
   - Button clicks: <100ms
   - Form submission: <500ms
   - Data updates: <1s

3. Stability
   - No crashes: 24h uptime
   - No data loss: All data persisted
   - No memory leaks: Stable memory
```

#### Compatibility Testing
```
Browsers:
- Chrome 120+
- Firefox 121+
- Safari 17+
- Edge 120+

Devices:
- Desktop (1920x1080)
- Tablet (768x1024)
- Mobile (375x667)

Operating Systems:
- macOS 13+
- Windows 11
- Ubuntu 22.04
- iOS 17+
- Android 13+
```

### 3.2 User Feedback Collection

#### Feedback Forms
```
1. Usability Survey
   - Ease of use (1-5)
   - Feature completeness (1-5)
   - Performance (1-5)
   - Design (1-5)
   - Overall satisfaction (1-5)

2. Bug Report Form
   - Description
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Screenshots/videos

3. Feature Request Form
   - Feature description
   - Use case
   - Priority (1-5)
   - Additional context
```

#### Metrics to Track
```
- User engagement time
- Feature usage frequency
- Error rates
- Support tickets
- User satisfaction score
- Net Promoter Score (NPS)
```

### 3.3 Issues & Resolution

#### Issue Tracking
```
Priority Levels:
- Critical: System down, data loss
- High: Major feature broken
- Medium: Minor feature issue
- Low: UI/UX improvement

Resolution Time:
- Critical: <1 hour
- High: <4 hours
- Medium: <1 day
- Low: <1 week
```

---

## Phase 4: Staging Validation (Days 15)

### 4.1 Final Checks

#### Deployment Checklist
- [ ] All tests passing
- [ ] No critical issues
- [ ] Performance acceptable
- [ ] Security audit passed
- [ ] User testing complete
- [ ] Documentation updated
- [ ] Rollback plan ready
- [ ] Monitoring active
- [ ] Alerts configured
- [ ] Team trained

#### Sign-off
```
Backend Lead: ___________
Frontend Lead: ___________
DevOps Lead: ___________
Security Lead: ___________
Product Manager: ___________
```

### 4.2 Production Readiness

#### Go/No-Go Decision
```
Criteria for Production Deployment:
✅ All critical tests passing
✅ No critical security issues
✅ Performance meets targets
✅ User testing successful
✅ Documentation complete
✅ Team trained
✅ Monitoring ready
✅ Rollback plan ready
```

---

## Monitoring Dashboard Example

```
╔════════════════════════════════════════════════════════════════╗
║                    CHATAPP STAGING DASHBOARD                  ║
╠════════════════════════════════════════════════════════════════╣
║                                                                ║
║  Status: ✅ HEALTHY                                           ║
║  Uptime: 99.95% (14 days)                                     ║
║  Users: 150 active                                            ║
║                                                                ║
║  ┌─ Performance ─────────────────────────────────────────┐   ║
║  │ Request Rate: 1,234 req/s                             │   ║
║  │ Latency p95: 45ms                                     │   ║
║  │ Error Rate: 0.02%                                     │   ║
║  │ Cache Hit: 87%                                        │   ║
║  └───────────────────────────────────────────────────────┘   ║
║                                                                ║
║  ┌─ Resources ───────────────────────────────────────────┐   ║
║  │ CPU: 35% (4 cores)                                    │   ║
║  │ Memory: 4.2GB / 8GB (52%)                             │   ║
║  │ Disk: 45GB / 100GB (45%)                              │   ║
║  │ Network: 50 Mbps / 100 Mbps (50%)                     │   ║
║  └───────────────────────────────────────────────────────┘   ║
║                                                                ║
║  ┌─ Database ────────────────────────────────────────────┐   ║
║  │ Connections: 45 / 100                                 │   ║
║  │ Query Time p95: 25ms                                  │   ║
║  │ Replication Lag: 0ms                                  │   ║
║  │ Backup Status: ✅ Last 1h ago                         │   ║
║  └───────────────────────────────────────────────────────┘   ║
║                                                                ║
║  ┌─ Alerts ──────────────────────────────────────────────┐   ║
║  │ Critical: 0                                           │   ║
║  │ Warning: 1 (High memory on staging-2)                 │   ║
║  │ Info: 3                                               │   ║
║  └───────────────────────────────────────────────────────┘   ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
```

---

**Status**: Ready for Implementation  
**Timeline**: 2 weeks  
**Resources Required**: 2-3 engineers + 1 DevOps  
**Expected Outcome**: Production-ready staging environment with comprehensive monitoring
