import { CodeBlock } from '../components/CodeBlock'

export default function SchemasPage() {
  return (
    <div className="min-h-screen bg-white">
      {/* Page Header */}
      <section className="bg-gradient-to-br from-pistachio-50 to-white py-12 md:py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-4">JSON Schemas</h1>
          <p className="text-xl text-gray-600">
            Leverage JSON schemas for configuration validation and IDE auto-completion.
          </p>
        </div>
      </section>

      {/* Content */}
      <section className="py-16 md:py-24">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 space-y-12">
          
          {/* Available Schemas */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Available Schemas</h2>
            <p className="text-gray-600 mb-6">
              naechste provides JSON schemas for configuration validation. Download and use them in your IDE for full auto-completion and validation.
            </p>
            
            <div className="space-y-4">
              <div className="border border-gray-200 rounded-lg p-4 hover:border-pistachio-300 transition">
                <h3 className="font-semibold text-gray-900 mb-2">Latest Version</h3>
                <a 
                  href="https://zeropaper.github.io/naechste/schemas/naechste.json" 
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-pistachio-600 hover:text-pistachio-700 font-medium break-all"
                >
                  naechste.json
                </a>
                <p className="text-gray-600 text-sm mt-1">Always points to the latest stable schema</p>
              </div>

              <div className="border border-gray-200 rounded-lg p-4 hover:border-pistachio-300 transition">
                <h3 className="font-semibold text-gray-900 mb-2">Version 0.1.2-beta.2</h3>
                <a 
                  href="https://zeropaper.github.io/naechste/schemas/naechste-0.1.2-beta.2.json" 
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-pistachio-600 hover:text-pistachio-700 font-medium break-all"
                >
                  naechste-0.1.2-beta.2.json
                </a>
                <p className="text-gray-600 text-sm mt-1">Pinned version schema - recommended for reproducibility</p>
              </div>
            </div>
          </div>

          {/* Using Schemas */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Using Schemas in Your Projects</h2>

            <div className="space-y-8">
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">VS Code Setup</h3>
                <p className="text-gray-600 mb-3">Add the <code className="bg-gray-100 px-2 py-1 rounded">$schema</code> field to your naechste config:</p>
                <CodeBlock
                  code={`{
  "$schema": "https://zeropaper.github.io/naechste/schemas/naechste.json",
  "rules": {
    "server_side_exports": { "severity": "error" }
  }
}`}
                  title="naechste.json"
                />
                <p className="text-gray-600 text-sm mt-3">
                  VS Code will automatically fetch the schema and provide:
                </p>
                <ul className="text-gray-600 text-sm mt-2 space-y-1 ml-4">
                  <li>• Auto-completion for all available rules</li>
                  <li>• Property validation with error highlighting</li>
                  <li>• IntelliSense on hover for descriptions</li>
                  <li>• Real-time validation as you type</li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">Manual Schema Download</h3>
                <p className="text-gray-600 mb-3">For offline use or air-gapped environments:</p>
                <CodeBlock
                  code={`# Download to your project
curl -o naechste.schema.json \\
  https://zeropaper.github.io/naechste/schemas/naechste.json

# Then reference locally
{
  "$schema": "./naechste.schema.json",
  "rules": { ... }
}`}
                  language="bash"
                  title="Download schema locally"
                />
              </div>

              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">IDE Support</h3>
                <p className="text-gray-600 mb-3">JSON schemas work with many modern editors:</p>
                <div className="grid md:grid-cols-2 gap-4">
                  <div className="text-gray-600 text-sm">
                    <span className="font-semibold">✓ Supported</span>
                    <ul className="mt-2 space-y-1">
                      <li>• VS Code</li>
                      <li>• WebStorm (JetBrains)</li>
                      <li>• IntelliJ IDEA</li>
                      <li>• Sublime Text (with plugins)</li>
                    </ul>
                  </div>
                  <div className="text-gray-600 text-sm">
                    <span className="font-semibold">✓ Works via</span>
                    <ul className="mt-2 space-y-1">
                      <li>• Native JSON schema support</li>
                      <li>• JSON language server</li>
                      <li>• LSP (Language Server Protocol)</li>
                      <li>• IDE-specific config files</li>
                    </ul>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Schema Contents */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Schema Contents</h2>
            <p className="text-gray-600 mb-4">
              The schema defines the complete structure for naechste configuration:
            </p>

            <div className="space-y-4">
              <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
                <h3 className="font-semibold text-gray-900 mb-2">Root Properties</h3>
                <ul className="text-gray-600 text-sm space-y-1">
                  <li><code className="bg-white px-2 py-1 rounded">$schema</code> — Optional reference to this schema</li>
                  <li><code className="bg-white px-2 py-1 rounded">rules</code> — Object containing all rules configuration</li>
                  <li><code className="bg-white px-2 py-1 rounded">extends</code> — Optional config file to extend from</li>
                </ul>
              </div>

              <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
                <h3 className="font-semibold text-gray-900 mb-2">Rule Properties</h3>
                <ul className="text-gray-600 text-sm space-y-1">
                  <li><code className="bg-white px-2 py-1 rounded">severity</code> — "error" or "warn" (default: "warn")</li>
                  <li><code className="bg-white px-2 py-1 rounded">enabled</code> — Boolean to enable/disable rule (default: true)</li>
                  <li><code className="bg-white px-2 py-1 rounded">options</code> — Rule-specific configuration object</li>
                </ul>
              </div>

              <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
                <h3 className="font-semibold text-gray-900 mb-2">Available Rules</h3>
                <ul className="text-gray-600 text-sm space-y-1">
                  <li><code className="bg-white px-2 py-1 rounded">server_side_exports</code> — Detect server-only code in client components</li>
                  <li><code className="bg-white px-2 py-1 rounded">component_nesting_depth</code> — Limit component nesting levels</li>
                  <li><code className="bg-white px-2 py-1 rounded">filename_style_consistency</code> — Enforce consistent file naming</li>
                  <li><code className="bg-white px-2 py-1 rounded">file_organization</code> — Enforce file organization patterns</li>
                </ul>
              </div>
            </div>
          </div>

          {/* Validation */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Validation</h2>
            <p className="text-gray-600 mb-4">
              Validate your configuration file against the schema before deployment:
            </p>

            <CodeBlock
              code={`# Using ajv-cli (npm/yarn)
npm install -g ajv-cli
ajv validate -s naechste.json -d naechste.config.json

# Using jsonschema (Python 3)
python3 -m json.tool naechste.config.json > /dev/null &&
  python3 -c "import json, sys; json.load(open(sys.argv[1]))" naechste.config.json

# In CI/CD pipelines, naechste auto-validates
naechste --config naechste.json .`}
              language="bash"
              title="Validation examples"
            />
          </div>

          {/* Tips */}
          <div className="bg-pistachio-50 border border-pistachio-200 rounded-lg p-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-3">💡 Pro Tips</h3>
            <ul className="text-gray-700 space-y-2 text-sm">
              <li><strong>Use latest schema for new projects:</strong> Always reference the main <code className="bg-white px-1 rounded">naechste.json</code> to get the newest features.</li>
              <li><strong>Pin version for reproducibility:</strong> In CI/CD, pin the versioned schema like <code className="bg-white px-1 rounded">naechste-0.1.2-beta.2.json</code>.</li>
              <li><strong>Enable in your IDE:</strong> Add <code className="bg-white px-1 rounded">$schema</code> field to catch config errors early.</li>
              <li><strong>Comments in JSON:</strong> naechste supports JSONC (JSON with Comments) for documentation in your config.</li>
            </ul>
          </div>

        </div>
      </section>
    </div>
  )
}
