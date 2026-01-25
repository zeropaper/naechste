#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const REPO = 'zeropaper/naechste';
const BIN_DIR = path.join(__dirname, '..', 'bin');
const BINARY_NAME = 'naechste';

// Platform and architecture mapping
function getPlatform() {
  const type = process.platform;
  const arch = process.arch;

  const platforms = {
    'darwin-x64': 'darwin-x86_64',
    'darwin-arm64': 'darwin-aarch64',
    'linux-x64': 'linux-x86_64',
    'linux-arm64': 'linux-aarch64',
    'win32-x64': 'windows-x86_64',
  };

  const key = `${type}-${arch}`;
  const platform = platforms[key];

  if (!platform) {
    throw new Error(
      `Unsupported platform: ${type} ${arch}. Supported platforms: ${Object.keys(platforms).join(', ')}`
    );
  }

  return platform;
}

// Get package version
function getVersion() {
  const packageJson = require('../package.json');
  return packageJson.version;
}

// Download file from URL
function download(url, dest) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading ${url}...`);
    
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        return download(response.headers.location, dest).then(resolve).catch(reject);
      }
      
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${response.statusCode} ${response.statusMessage}`));
        return;
      }

      response.pipe(file);
      file.on('finish', () => {
        file.close(() => resolve());
      });
    }).on('error', (err) => {
      fs.unlink(dest, () => {});
      reject(err);
    });
  });
}

// Main installation logic
async function install() {
  try {
    const platform = getPlatform();
    const version = getVersion();
    const isWindows = process.platform === 'win32';
    const binaryExt = isWindows ? '.exe' : '';
    const archiveExt = isWindows ? '.zip' : '.tar.gz';
    
    // Create bin directory if it doesn't exist
    if (!fs.existsSync(BIN_DIR)) {
      fs.mkdirSync(BIN_DIR, { recursive: true });
    }

    const binaryPath = path.join(BIN_DIR, BINARY_NAME + binaryExt);
    
    // Download URL pattern: https://github.com/{owner}/{repo}/releases/download/v{version}/naechste-{platform}.{ext}
    const archiveName = `${BINARY_NAME}-${platform}${archiveExt}`;
    const downloadUrl = `https://github.com/${REPO}/releases/download/v${version}/${archiveName}`;
    const archivePath = path.join(BIN_DIR, archiveName);

    console.log(`Installing naechste v${version} for ${platform}...`);
    
    // Download the archive
    await download(downloadUrl, archivePath);
    
    // Extract the binary
    console.log('Extracting binary...');
    if (isWindows) {
      // For Windows, use tar (available in Windows 10+) or fall back to unzip
      try {
        execSync(`tar -xf "${archivePath}" -C "${BIN_DIR}"`, { stdio: 'inherit' });
      } catch (err) {
        console.error('Failed to extract with tar, trying PowerShell...');
        execSync(
          `powershell -command "Expand-Archive -Path '${archivePath}' -DestinationPath '${BIN_DIR}' -Force"`,
          { stdio: 'inherit' }
        );
      }
    } else {
      execSync(`tar -xzf "${archivePath}" -C "${BIN_DIR}"`, { stdio: 'inherit' });
    }

    // Clean up archive
    fs.unlinkSync(archivePath);

    // Make binary executable on Unix-like systems
    if (!isWindows) {
      fs.chmodSync(binaryPath, 0o755);
    }

    console.log(`✓ naechste installed successfully at ${binaryPath}`);
    
    // Verify installation
    try {
      const versionOutput = execSync(`"${binaryPath}" --version`, { encoding: 'utf8' });
      console.log(`✓ Verification: ${versionOutput.trim()}`);
    } catch (err) {
      console.warn('Warning: Could not verify installation');
    }

  } catch (error) {
    console.error('Installation failed:', error.message);
    console.error('\nPlease try one of the following:');
    console.error('1. Install from source: cargo install --git https://github.com/zeropaper/naechste');
    console.error('2. Download binary manually from: https://github.com/zeropaper/naechste/releases');
    console.error('3. Build from source: git clone https://github.com/zeropaper/naechste && cd naechste && cargo build --release');
    process.exit(1);
  }
}

// Run installation
install();
