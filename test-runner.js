#!/usr/bin/env node

/**
 * Simple test runner for Solana security examples
 * Runs mock tests that don't require a local validator
 */

const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const examples = [
  '01_missing_account_validation',
  '02_authority_check_failure', 
  '03_unsafe_cpi',
  '04_arithmetic_overflow',
  '05_reinitialization_attack'
];

console.log('🚀 Running Solana Security Examples Tests\n');

let totalTests = 0;
let passedTests = 0;
let failedTests = 0;

for (const example of examples) {
  console.log(`\n📁 Testing ${example}...`);
  console.log('='.repeat(50));
  
  try {
    // Check if example directory exists
    if (!fs.existsSync(example)) {
      console.log(`❌ Directory ${example} not found`);
      failedTests++;
      continue;
    }
    
    // Check if package.json exists
    const packageJsonPath = path.join(example, 'package.json');
    if (!fs.existsSync(packageJsonPath)) {
      console.log(`❌ package.json not found in ${example}`);
      failedTests++;
      continue;
    }
    
    // Install dependencies if node_modules doesn't exist
    const nodeModulesPath = path.join(example, 'node_modules');
    if (!fs.existsSync(nodeModulesPath)) {
      console.log('📦 Installing dependencies...');
      execSync('npm install', { 
        cwd: example, 
        stdio: 'inherit',
        timeout: 60000 
      });
    }
    
    // Run tests
    console.log('🧪 Running tests...');
    execSync('npm test', { 
      cwd: example, 
      stdio: 'inherit',
      timeout: 30000 
    });
    
    console.log(`✅ ${example} tests passed`);
    passedTests++;
    totalTests++;
    
  } catch (error) {
    console.log(`❌ ${example} tests failed:`, error.message);
    failedTests++;
    totalTests++;
  }
}

console.log('\n' + '='.repeat(60));
console.log('📊 TEST SUMMARY');
console.log('='.repeat(60));
console.log(`Total Examples: ${totalTests}`);
console.log(`✅ Passed: ${passedTests}`);
console.log(`❌ Failed: ${failedTests}`);

if (failedTests === 0) {
  console.log('\n🎉 All tests passed! The examples are working correctly.');
  process.exit(0);
} else {
  console.log('\n⚠️  Some tests failed. Check the output above for details.');
  process.exit(1);
}