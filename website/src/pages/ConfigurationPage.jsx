import { CodeBlock } from '../components/CodeBlock'

export default function ConfigurationPage() {
  return (
    <div className="min-h-screen bg-white">
      {/* Page Header */}
      <section className="bg-gradient-to-br from-pistachio-50 to-white py-12 md:py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-4">Configuration</h1>
          <p className="text-xl text-gray-600">
            Learn how to configure naechste for your project and customize rules.
          </p>
        </div>
      </section>

      {/* Content */}
      <section className="py-16 md:py-24">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 space-y-12">
          
          {/* Configuration Files */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Configuration Files</h2>
            <p className="text-gray-600 mb-4">
              naechste supports multiple configuration file formats. Place any of these in your project root:
            </p>
            <CodeBlock
              code={`naechste.json    # JSON with optional comments (JSONC)
naechste.jsonc   # JSON with comments
naechste.yaml    # YAML format
naechste.yml     # YAML format`}
              language="bash"
            />
            <p className="text-gray-600 mt-4">
              The tool automatically detects the configuration file when no <code className="bg-gray-100 px-2 py-1 rounded">--config</code> is provided.
            </p>
          </div>

          {/* Severity Levels */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Severity Levels</h2>
            <p className="text-gray-600 mb-4">Each rule can have a severity level that determines CI/CD behavior:</p>
            
            <div className="grid md:grid-cols-2 gap-6 mb-6">
              <div className="bg-yellow-50 border-l-4 border-yellow-400 p-4 rounded">
                <h3 className="font-semibold text-gray-900 mb-2">warn</h3>
                <p className="text-gray-600 text-sm">Reports issues but doesn't fail CI (exit code 0)</p>
              </div>
              <div className="bg-red-50 border-l-4 border-red-400 p-4 rounded">
                <h3 className="font-semibold text-gray-900 mb-2">error</h3>
                <p className="text-gray-600 text-sm">Reports issues and fails CI (exit code 1)</p>
              </div>
            </div>

            <CodeBlock
              code={`{
  "rules": {
    "server_side_exports": {
      "severity": "error"    // Fails CI
    },
    "component_nesting_depth": {
      "severity": "warn"     // Warnings only
    }
  }
}`}
              title="naechste.json"
            />
          </div>

          {/* Per-Rule Configuration */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Per-Rule Configuration</h2>

            <div className="space-y-8">
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">Component Nesting Depth</h3>
                <CodeBlock
                  code={`{
  "component_nesting_depth": {
    "severity": "warn",
    "options": {
      "max_nesting_depth": 3
    }
  }
}`}
                  title="naechste.json"
                />
                <p className="text-gray-600 text-sm mt-3">
                  Valid values: 1-10. Default: 3
                </p>
              </div>

              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">Filename Style Consistency</h3>
                <CodeBlock
                  code={`{
  "filename_style_consistency": {
    "severity": "warn",
    "options": {
      "filename_style": "kebab-case"
    }
  }
}`}
                  title="naechste.json"
                />
                <p className="text-gray-600 text-sm mt-3">
                  Options: <code className="bg-gray-100 px-1 rounded">kebab-case</code> | <code className="bg-gray-100 px-1 rounded">PascalCase</code> | <code className="bg-gray-100 px-1 rounded">camelCase</code> | <code className="bg-gray-100 px-1 rounded">snake_case</code>. Default: <code className="bg-gray-100 px-1 rounded">kebab-case</code>
                </p>
              </div>

              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">File Organization</h3>
                <CodeBlock
                  code={`{
  "file_organization": {
    "severity": "warn",
    "options": {
      "file_organization_checks": [
        {
          "id": "require-test-files",
          "match": {
            "glob": "app/components/**/*.tsx",
            "exclude_glob": ["**/page.tsx"]
          },
          "require": [
            {
              "kind": "sibling_exact",
              "name": "Component.test.tsx"
            }
          ]
        }
      ]
    }
  }
}`}
                  title="naechste.json"
                />
              </div>
            </div>
          </div>

          {/* File Organization Details */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">File Organization Checks</h2>
            <p className="text-gray-600 mb-4">
              Advanced file organization checks support sibling requirements, location enforcement, and import-based conditions.
            </p>

            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">Check Structure</h3>
                <CodeBlock
                  code={`{
  "id": "unique-id",                    // Required
  "description": "Optional description", // Optional
  "match": {
    "glob": "**/*.tsx",                  // Required
    "exclude_glob": ["**/page.tsx"]      // Optional
  },
  "require": [                            // Optional
    {
      "kind": "sibling_exact",
      "name": "Component.test.tsx"
    }
  ],
  "when_imported_by": {                  // Optional
    "importer_glob": "app/**",
    "import_path_matches": ["^@/components/ui/"]
  },
  "enforce_location": {                  // Optional
    "must_be_under": ["components/ui"],
    "message": "Custom error message"
  }
}`}
                  title="Check structure"
                />
              </div>

              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">Sibling File Requirements</h3>
                <CodeBlock
                  code={`{
  "require": [
    {
      "kind": "sibling_exact",
      "name": "Component.test.tsx"  // Exact filename
    },
    {
      "kind": "sibling_glob",
      "glob": "*.stories.tsx"       // Glob pattern
    }
  ]
}`}
                  title="Sibling requirements"
                />
              </div>

              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">Location Enforcement</h3>
                <CodeBlock
                  code={`{
  "enforce_location": {
    "must_be_under": [
      "components/ui",
      "components/shared"  // Multiple allowed roots
    ],
    "message": "UI components must live under components/ui or components/shared"
  }
}`}
                  title="Location enforcement"
                />
              </div>
            </div>
          </div>

          {/* Complete Example */}
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-6">Complete Example</h2>
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
        "max_nesting_depth": 4
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
        "file_organization_checks": [
          {
            "id": "components-need-tests",
            "match": {
              "glob": "app/components/**/*.tsx",
              "exclude_glob": ["**/page.tsx", "**/layout.tsx"]
            },
            "require": [
              {
                "kind": "sibling_exact",
                "name": "Component.test.tsx"
              }
            ]
          },
          {
            "id": "ui-components-location",
            "match": { "glob": "**/*.tsx" },
            "when_imported_by": {
              "importer_glob": "app/**",
              "import_path_matches": ["^@/components/ui/"]
            },
            "enforce_location": {
              "must_be_under": ["components/ui"],
              "message": "UI components must live under components/ui"
            }
          }
        ]
      }
    }
  }
}`}
              title="naechste.json - Complete example"
            />
          </div>

        </div>
      </section>
    </div>
  )
}
