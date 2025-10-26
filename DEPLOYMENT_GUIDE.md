# Deployment & Testing Guide

**Date**: October 26, 2025  
**Status**: Ready for Integration Testing

## Quick Start

### Backend Setup

```bash
# Navigate to project root
cd /Users/yingkitw/Desktop/productivity\ project/chatapp

# Build backend
cargo build

# Run tests
cargo test

# Start server
cargo run --bin chatapp-cli -- run examples/demo.rs
```

### Frontend Setup

```bash
# Navigate to frontend
cd frontend

# Install dependencies (already done)
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Run integration tests
npm run test:integration
```

## Integration Testing

### Manual Testing Steps

1. **Start Backend Server**
   ```bash
   cargo run --bin chatapp-cli -- run examples/demo.rs
   ```
   Expected: Server starts on `http://localhost:8000`

2. **Start Frontend Dev Server**
   ```bash
   cd frontend && npm run dev
   ```
   Expected: Frontend available at `http://localhost:3000`

3. **Test WebSocket Connection**
   - Open browser console
   - Check for connection messages
   - Verify no errors in console

4. **Test Proto Serialization**
   - Interact with widgets
   - Check network tab for binary messages
   - Verify message size (~50% of JSON)

5. **Test State Management**
   - Change input values
   - Verify state updates in React DevTools
   - Check for proper debouncing

6. **Test Event Propagation**
   - Click buttons
   - Submit forms
   - Verify server receives events

### Automated Integration Tests

```bash
# Run integration tests
cd frontend
npm run test:integration

# Expected output:
# ✅ Proto Initialization: Xms
# ✅ WebSocket Connection: Xms
# ✅ State Management: Xms
# ✅ Event Propagation: Xms
# ✅ Message Serialization: Xms
# Total: 5/5 passed
```

## Performance Testing

### Metrics to Monitor

1. **Bundle Size**
   - CSS: ~2KB gzipped
   - JS: ~77KB gzipped
   - Total: ~80KB gzipped

2. **Load Time**
   - Initial load: <1s
   - Time to interactive: <2s
   - First contentful paint: <500ms

3. **Runtime Performance**
   - Proto serialization: <1ms
   - State updates: <5ms
   - React render: <16ms (60fps)

### Performance Profiling

```bash
# Enable performance monitoring
# In browser console:
import { monitorWebVitals, performanceTracker } from './performance.ts';
monitorWebVitals();
performanceTracker.log();
```

## Deployment Checklist

### Pre-Deployment

- [ ] All tests passing (backend: 30/30, frontend: integration tests)
- [ ] Build succeeds without errors
- [ ] No console errors or warnings
- [ ] Performance metrics within targets
- [ ] Documentation updated
- [ ] Environment variables configured

### Deployment Steps

1. **Build Backend**
   ```bash
   cargo build --release
   ```

2. **Build Frontend**
   ```bash
   cd frontend && npm run build
   ```

3. **Deploy Backend**
   ```bash
   # Copy to server
   scp target/release/chatapp-cli user@server:/app/
   
   # Start service
   systemctl start chatapp
   ```

4. **Deploy Frontend**
   ```bash
   # Copy dist to web server
   scp -r frontend/dist/* user@server:/var/www/chatapp/
   
   # Configure nginx/apache
   # Point to dist/index.html for SPA routing
   ```

### Post-Deployment

- [ ] Verify backend is running
- [ ] Verify frontend is accessible
- [ ] Test WebSocket connection
- [ ] Monitor error logs
- [ ] Check performance metrics

## Environment Configuration

### Backend (.env)

```env
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8000
LOG_LEVEL=info

# Database (future)
DATABASE_URL=postgres://user:pass@localhost/chatapp

# Security
CORS_ORIGIN=http://localhost:3000
```

### Frontend (.env)

```env
# API
VITE_API_URL=http://localhost:8000
VITE_WS_URL=ws://localhost:8000

# Features
VITE_ENABLE_ANALYTICS=true
VITE_ENABLE_PERFORMANCE_MONITORING=true
```

## Troubleshooting

### WebSocket Connection Issues

**Problem**: WebSocket connection fails
**Solution**:
1. Check backend is running on port 8000
2. Check CORS settings
3. Check firewall rules
4. Check browser console for errors

### Proto Message Errors

**Problem**: Proto deserialization fails
**Solution**:
1. Verify proto files are loaded
2. Check message format
3. Check version compatibility
4. Enable debug logging

### Performance Issues

**Problem**: Slow rendering
**Solution**:
1. Check React DevTools for unnecessary re-renders
2. Enable performance monitoring
3. Check network tab for large messages
4. Profile with Chrome DevTools

### State Sync Issues

**Problem**: UI not updating
**Solution**:
1. Check WebSocket connection
2. Verify state manager is initialized
3. Check event listeners are registered
4. Enable debug logging in state manager

## Monitoring & Logging

### Backend Logging

```rust
// Enable debug logging
RUST_LOG=debug cargo run
```

### Frontend Logging

```typescript
// Enable performance tracking
import { performanceTracker } from './performance.ts';
performanceTracker.log();

// Enable debug logging
localStorage.setItem('DEBUG', 'chatapp:*');
```

## Rollback Procedure

### If Backend Deployment Fails

```bash
# Revert to previous version
systemctl stop chatapp
cp /app/chatapp-cli.backup /app/chatapp-cli
systemctl start chatapp
```

### If Frontend Deployment Fails

```bash
# Revert to previous version
rm -rf /var/www/chatapp/*
cp -r /var/www/chatapp.backup/* /var/www/chatapp/
```

## Scaling Considerations

### Horizontal Scaling

1. **Load Balancer**
   - Use nginx/HAProxy
   - Route to multiple backend instances
   - Sticky sessions for WebSocket

2. **Session Management**
   - Use Redis for session store
   - Share session state across instances

3. **Database**
   - Use PostgreSQL for persistence
   - Implement connection pooling

### Vertical Scaling

1. **Backend**
   - Increase CPU cores
   - Increase memory
   - Optimize database queries

2. **Frontend**
   - Enable CDN for static assets
   - Implement service workers
   - Use edge caching

## Security Considerations

### HTTPS/WSS

```nginx
# Nginx configuration
server {
    listen 443 ssl;
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

### CORS Configuration

```rust
// Backend CORS settings
let cors = CorsLayer::permissive()
    .allow_origin("https://yourdomain.com".parse()?)
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE]);
```

### Rate Limiting

```rust
// Implement rate limiting middleware
let rate_limit = RateLimitLayer::new(
    100,  // requests
    Duration::from_secs(60),  // per minute
);
```

## Maintenance

### Regular Tasks

- [ ] Monitor error logs daily
- [ ] Check performance metrics weekly
- [ ] Update dependencies monthly
- [ ] Run security audit monthly
- [ ] Backup data daily

### Upgrade Procedure

1. Test in staging environment
2. Backup production data
3. Deploy during low-traffic period
4. Monitor for errors
5. Rollback if issues occur

## Support & Documentation

- **Backend Docs**: See ARCHITECTURE.md
- **Frontend Docs**: See frontend/README.md
- **API Docs**: See PHASE_4_INTEGRATION.md
- **Issues**: Check GitHub issues
- **Contact**: See GETTING_STARTED.md

---

**Last Updated**: October 26, 2025  
**Status**: Ready for Production  
**Next Review**: After first deployment
