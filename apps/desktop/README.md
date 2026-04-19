# Zaroxi Desktop App

This is the Tauri 2-based desktop application for Zaroxi Studio.

## Quick Start

```bash
# Navigate to the desktop app directory
cd apps/desktop

# Install dependencies
npm install

# Build Rust dependencies (from the root directory)
cd ../..
cargo build --workspace

# Return to desktop app and start development
cd apps/desktop
npm run tauri dev
```

## Prerequisites

- Node.js 18+ and npm
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Tauri CLI will be installed locally via devDependencies

## Setup

1. Make sure you're in the correct directory:
   ```bash
   pwd  # Should show: .../zaroxi/apps/desktop
   ```

2. Install npm dependencies:
   ```bash
   npm install
   ```

3. Build the Rust workspace:
   ```bash
   cd ../..
   cargo build --workspace
   cd apps/desktop
   ```

## Development

Run the app in development mode:
```bash
npm run tauri dev
```

This will start:
- Frontend development server on http://localhost:1420
- Tauri application with hot reload

For frontend-only development (without Tauri):
```bash
npm run dev
```

## Building for Production

```bash
npm run tauri build
```

Or use the build script:
```bash
./build.sh
```

The built application will be in `src-tauri/target/release/`.

## Project Structure

- `src/` - React TypeScript frontend
- `src-tauri/` - Rust backend
- `public/` - Static assets

## Troubleshooting

### Common Issues

1. **"package.json not found" error**: You're not in the `apps/desktop` directory
   ```bash
   cd apps/desktop
   ```

2. **"npm install" fails**: Check Node.js version (should be 18+)
   ```bash
   node --version
   ```

3. **Rust dependencies not found**: Build from the root
   ```bash
   cd ../..
   cargo build --workspace
   cd apps/desktop
   ```

4. **Tauri not found**: It's installed as a dev dependency, no need for global install

### Development Tips

- Use `npm run dev` for frontend-only development (without Tauri)
- Use `npm run tauri dev` for full application with Rust backend
- Check browser console for frontend errors
- Check terminal for Rust backend errors
- If you see "Initializing..." forever, check the browser console for errors

## First Run Checklist

1. ✅ Navigate to `apps/desktop`
2. ✅ Run `npm install`
3. ✅ Run `cd ../.. && cargo build --workspace`
4. ✅ Run `cd apps/desktop && npm run tauri dev`

If you still have issues:

### Manual Setup
```bash
# 1. Navigate to the desktop app
cd apps/desktop

# 2. Install npm dependencies
npm install

# 3. Build Rust dependencies (from the root)
cd ../..
cargo build --workspace

# 4. Return and start the app
cd apps/desktop
npm run tauri dev
```

### If you get "run.sh not found" error:
The run.sh script should be created automatically. If not, you can:
1. Make it executable: `chmod +x run.sh`
2. Run it: `./run.sh`

Or just follow the manual setup steps above.

### Common Issues:
1. **"package.json not found"**: You're not in the right directory
2. **"npm install fails"**: Check Node.js version (18+ required)
3. **"cargo build fails"**: Make sure Rust is installed (rustup.rs)
4. **"Tauri not found"**: It's installed as a dev dependency, no global install needed

### Quick Test:
Run the setup check to verify your environment:
```bash
node check-setup.js
```

## Quick Start (Easiest Way)

From **anywhere** in the zaroxi repository:

```bash
# Make scripts executable (first time only)
./apps/desktop/fix-permissions.sh

# Run the desktop app
./apps/desktop/run.sh
```

The scripts will automatically:
1. Find the correct directories
2. Install npm dependencies if needed
3. Build Rust dependencies if needed
4. Start the development server

## Alternative Methods

### Method 1: Using scripts from anywhere
```bash
# From the zaroxi repository root
./apps/desktop/run.sh      # Start development
./apps/desktop/setup.sh    # Install dependencies
./apps/desktop/build.sh    # Build for production
```

### Method 2: Manual steps
```bash
# 1. Navigate to desktop app
cd apps/desktop

# 2. Install dependencies (first time)
npm install

# 3. Build Rust dependencies (from root)
cd ../..
cargo build --workspace

# 4. Start development
cd apps/desktop
npm run tauri dev
```

### Method 3: Using npm directly
```bash
cd apps/desktop
npm run setup    # Install dependencies
npm run tauri dev # Start development
```

## Script Reference

All scripts can be run from **anywhere** in the zaroxi repository:

- `./apps/desktop/run.sh` - Start development server
- `./apps/desktop/start.sh` - Alternative start script
- `./apps/desktop/setup.sh` - Install npm dependencies
- `./apps/desktop/build.sh` - Build for production
- `./apps/desktop/fix-permissions.sh` - Make scripts executable

## Troubleshooting

### "No such file or directory" errors
Make sure you're in the zaroxi repository root:
```bash
pwd  # Should show: /home/yourname/Work/zaroxi
ls -la apps/desktop/  # Should show the scripts
```

### Fix permissions (first time)
```bash
./apps/desktop/fix-permissions.sh
```

### Check setup
```bash
cd apps/desktop
node check-setup.js
```

### Common Issues:
1. **"package.json not found"**: Run scripts from zaroxi repository root
2. **"npm install fails"**: Node.js 18+ required
3. **"cargo build fails"**: Install Rust via rustup.rs
4. **Scripts not executable**: Run `./apps/desktop/fix-permissions.sh`
