import { CodeBlock } from '../components/CodeBlock'
import { ArrowRight } from 'lucide-react'
import { Link } from 'react-router'

export default function GettingStartedPage() {
  return (
    <div className="min-h-screen bg-white">
      {/* Page Header */}
      <section className="bg-gradient-to-br from-pistachio-50 to-white py-12 md:py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-4">Getting Started</h1>
          <p className="text-xl text-gray-600">
            Install naechste, initialize your project, and start enforcing conventions in minutes.
          </p>
        </div>
      </section>

      {/* Content */}
      <section className="py-16 md:py-24">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 space-y-12">
          
          {/* Installation */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Installation</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">Via npm/pnpm/yarn</h3>
                <p className="text-gray-600 mb-4">Install naechste as a dev dependency in your project:</p>
                <CodeBlock code="pnpm add -D naechste" language="bash" />
              </div>

              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">From source (Rust)</h3>
                <p className="text-gray-600 mb-4">Build directly from the GitHub repository:</p>
                <CodeBlock code={`git clone https://github.com/zeropaper/naechste
cd naechste
cargo install --path .`} language="bash" />
              </div>
            </div>
          </div>

          {/* Initialization */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Initialization</h2>
            <p className="text-gray-600 mb-4">Initialize naechste in your project:</p>
            <CodeBlock code="pnpm naechste init" language="bash" />
            <p className="text-gray-600 mt-4">
              This creates a <code className="bg-gray-100 px-2 py-1 rounded">naechste.json</code> with sensible defaults.
            </p>
          </div>

          {/* First Run */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">First Run</h2>
            <p className="text-gray-600 mb-4">Run the linter on your project:</p>
            <CodeBlock code="pnpm naechste" language="bash" />
            <p className="text-gray-600 mt-4">
              naechste will scan your project and report any violations of the configured rules.
            </p>
          </div>

          {/* Configuration */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Basic Configuration</h2>
            <p className="text-gray-600 mb-4">Edit your <code className="bg-gray-100 px-2 py-1 rounded">naechste.json</code> to customize rules:</p>
            <CodeBlock
              code={`{
  "$schema": "https://zeropaper.github.io/naechste/schemas/naechste.json",
  "rules": {
    "server_side_exports": {
      "severity": "error"
    },
    "component_nesting_depth": {
      "severity": "warn",
      "options": {
        "max_nesting_depth": 3
      }
    },
    "filename_style_consistency": {
      "severity": "warn",
      "options": {
        "filename_style": "kebab-case"
      }
    },
    "file_organization": {
      "severity": "warn",
      "options": {
        "file_organization_checks": []
      }
    }
  }
}`}
              title="naechste.json"
            />
          </div>

          {/* CLI Options */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">CLI Options</h2>
            <CodeBlock
              code={`# Lint current directory
naechste

# Lint specific directory
naechste /path/to/nextjs/project

# Output JSON format (for CI/CD)
naechste --format json

# Use custom config file  
naechste --config my-config.json

# Show help
naechste --help`}
              language="bash"
            />
          </div>

          {/* CI/CD Integration */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">CI/CD Integration</h2>
            <p className="text-gray-600 mb-4">Add naechste to your GitHub Actions workflow:</p>
            <CodeBlock
              code={`name: naechste Lint
on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '18'
      - run: npm install
      - run: npx naechste --format json`}
              language="yaml"
              title=".github/workflows/naechste-lint.yml"
            />
          </div>

          {/* Next Steps */}
          <div className="bg-pistachio-50 border-l-4 border-pistachio-400 p-6 rounded">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Next Steps</h3>
            <ul className="space-y-3">
              <li className="flex items-start gap-3">
                <ArrowRight className="w-5 h-5 text-pistachio-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-semibold text-gray-900">
                    <Link to="/docs/rules" className="text-pistachio-600 hover:underline">Explore all rules</Link>
                  </p>
                  <p className="text-gray-600 text-sm">Learn what each rule does and how to use it.</p>
                </div>
              </li>
              <li className="flex items-start gap-3">
                <ArrowRight className="w-5 h-5 text-pistachio-600 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-semibold text-gray-900">
                    <Link to="/docs/configuration" className="text-pistachio-600 hover:underline">Advanced configuration</Link>
                  </p>
                  <p className="text-gray-600 text-sm">Customize rules and severity levels for your project.</p>
                </div>
              </li>
            </ul>
          </div>

        </div>
      </section>
    </div>
  )
}
