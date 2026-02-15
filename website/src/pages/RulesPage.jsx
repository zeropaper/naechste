import { CodeBlock, ExamplePair } from '../components/CodeBlock'
import { AlertCircle } from 'lucide-react'

export default function RulesPage() {
  return (
    <div className="min-h-screen bg-white">
      {/* Page Header */}
      <section className="bg-gradient-to-br from-pistachio-50 to-white py-12 md:py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-4">Linting Rules</h1>
          <p className="text-xl text-gray-600">
            naechste includes four powerful rules to enforce consistent file structure and naming conventions.
          </p>
        </div>
      </section>

      {/* Rules */}
      <section className="py-16 md:py-24">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 space-y-20">
          
          {/* Rule 1: Server-Side Exports */}
          <div id="server-side-exports">
            <div className="mb-8">
              <h2 className="text-3xl font-bold text-gray-900 mb-2">1. Server-Side Exports</h2>
              <p className="text-gray-600 text-lg">ID: <code className="bg-gray-100 px-2 py-1 rounded">server-side-exports</code></p>
            </div>
            
            <div className="bg-blue-50 border-l-4 border-blue-400 p-6 mb-8 rounded">
              <p className="text-blue-900">
                <span className="font-semibold">Purpose:</span> Detects server-side only exports (like <code className="bg-white px-1 rounded">getServerSideProps</code>, <code className="bg-white px-1 rounded">getStaticProps</code>) in client components (files with <code className="bg-white px-1 rounded">'use client'</code>).
              </p>
            </div>

            <div className="space-y-4 mb-8">
              <h3 className="text-lg font-semibold text-gray-900">What it catches:</h3>
              <ul className="space-y-2 text-gray-600">
                <li className="flex gap-3">
                  <span className="text-red-500">❌</span>
                  <span><code className="bg-gray-100 px-2 py-1 rounded text-sm">getServerSideProps</code> exported from a client component</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-red-500">❌</span>
                  <span><code className="bg-gray-100 px-2 py-1 rounded text-sm">getStaticProps</code> exported from a client component</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-red-500">❌</span>
                  <span><code className="bg-gray-100 px-2 py-1 rounded text-sm">getStaticPaths</code> exported from a client component</span>
                </li>
                <li className="flex gap-3">
                  <span className="text-red-500">❌</span>
                  <span><code className="bg-gray-100 px-2 py-1 rounded text-sm">getInitialProps</code> exported from a client component</span>
                </li>
              </ul>
            </div>

            <ExamplePair
              badCode={`'use client'

export function UserProfile() {
  return <div>Profile</div>
}

export async function getServerSideProps() {
  return { props: {} }
}`}
              goodCode={`// Server component (no 'use client')
export async function getServerSideProps() {
  return { props: {} }
}

export function UserProfile({ data }) {
  return <div>{data}</div>
}`}
              description="Moving server exports to server components ensures they only run on the server."
            />
          </div>

          {/* Rule 2: Component Nesting Depth */}
          <div id="component-nesting-depth">
            <div className="mb-8">
              <h2 className="text-3xl font-bold text-gray-900 mb-2">2. Component Nesting Depth</h2>
              <p className="text-gray-600 text-lg">ID: <code className="bg-gray-100 px-2 py-1 rounded">component-nesting-depth</code></p>
            </div>
            
            <div className="bg-blue-50 border-l-4 border-blue-400 p-6 mb-8 rounded">
              <p className="text-blue-900">
                <span className="font-semibold">Purpose:</span> Enforces maximum folder nesting depth for components to keep the file structure readable and prevent deep directory hierarchies.
              </p>
            </div>

            <div className="space-y-4 mb-8">
              <h3 className="text-lg font-semibold text-gray-900">Configuration:</h3>
              <CodeBlock
                code={`{
  "rules": {
    "component_nesting_depth": {
      "severity": "warn",
      "options": {
        "max_nesting_depth": 3
      }
    }
  }
}`}
                title="naechste.json"
              />
            </div>

            <ExamplePair
              badCode={`app/
  features/
    dashboard/
      widgets/
        charts/
          line/
            LineChart.tsx  (depth: 6)
            LineChart.test.tsx`}
              goodCode={`app/
  features/
    dashboard/
      LineChart.tsx  (depth: 3)
      LineChart.test.tsx`}
              description="Keeping components at a reasonable depth improves navigation and discoverability. Default max depth is 3."
            />
          </div>

          {/* Rule 3: Filename Style Consistency */}
          <div id="filename-style-consistency">
            <div className="mb-8">
              <h2 className="text-3xl font-bold text-gray-900 mb-2">3. Filename Style Consistency</h2>
              <p className="text-gray-600 text-lg">ID: <code className="bg-gray-100 px-2 py-1 rounded">filename-style-consistency</code></p>
            </div>
            
            <div className="bg-blue-50 border-l-4 border-blue-400 p-6 mb-8 rounded">
              <p className="text-blue-900">
                <span className="font-semibold">Purpose:</span> Enforces consistent filename casing across the project. Choose one convention and stick with it.
              </p>
            </div>

            <div className="space-y-4 mb-8">
              <h3 className="text-lg font-semibold text-gray-900">Supported styles:</h3>
              <div className="grid md:grid-cols-2 gap-4">
                <div className="bg-gray-50 p-4 rounded">
                  <p className="font-semibold text-gray-900">kebab-case (default)</p>
                  <code className="text-sm text-gray-700">my-component.tsx</code>
                </div>
                <div className="bg-gray-50 p-4 rounded">
                  <p className="font-semibold text-gray-900">PascalCase</p>
                  <code className="text-sm text-gray-700">MyComponent.tsx</code>
                </div>
                <div className="bg-gray-50 p-4 rounded">
                  <p className="font-semibold text-gray-900">camelCase</p>
                  <code className="text-sm text-gray-700">myComponent.tsx</code>
                </div>
                <div className="bg-gray-50 p-4 rounded">
                  <p className="font-semibold text-gray-900">snake_case</p>
                  <code className="text-sm text-gray-700">my_component.tsx</code>
                </div>
              </div>
            </div>

            <div className="bg-yellow-50 border-l-4 border-yellow-400 p-6 mb-8 rounded">
              <p className="text-yellow-900">
                <span className="font-semibold">Note:</span> Special Next.js files (<code className="bg-white px-1 rounded">page</code>, <code className="bg-white px-1 rounded">layout</code>, <code className="bg-white px-1 rounded">template</code>, <code className="bg-white px-1 rounded">error</code>, etc.) are automatically skipped.
              </p>
            </div>

            <ExamplePair
              badCode={`app/
  components/
    MyButton.tsx      // ❌ PascalCase
    user-list.tsx     // ❌ kebab-case
    headerBar.tsx     // ❌ camelCase`}
              goodCode={`app/
  components/
    my-button.tsx     // ✅ kebab-case
    user-list.tsx     // ✅ kebab-case
    header-bar.tsx    // ✅ kebab-case`}
              description="Consistent naming makes it easy to predict filenames and navigate the codebase."
            />
          </div>

          {/* Rule 4: File Organization */}
          <div id="file-organization">
            <div className="mb-8">
              <h2 className="text-3xl font-bold text-gray-900 mb-2">4. File Organization</h2>
              <p className="text-gray-600 text-lg">ID: <code className="bg-gray-100 px-2 py-1 rounded">file-organization</code></p>
            </div>
            
            <div className="bg-blue-50 border-l-4 border-blue-400 p-6 mb-8 rounded">
              <p className="text-blue-900">
                <span className="font-semibold">Purpose:</span> Enforce custom file organization rules including companion files, location enforcement, and import-based checks.
              </p>
            </div>

            <div className="space-y-6 mb-8">
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Example 1: Require companion files</h3>
                <p className="text-gray-600 mb-4">Ensure every component has a test file:</p>
                <CodeBlock
                  code={`{
  "id": "component-needs-tests",
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
}`}
                  title="naechste.json configuration"
                />
              </div>

              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-4">Example 2: Enforce file location based on imports</h3>
                <p className="text-gray-600 mb-4">Ensure UI components live in the correct directory:</p>
                <CodeBlock
                  code={`{
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
}`}
                  title="naechste.json configuration"
                />
              </div>
            </div>

            <ExamplePair
              badCode={`app/
  components/
    Button.tsx
    Button.stories.tsx
    Button.test.tsx
    (separate files scattered)`}
              goodCode={`app/
  components/
    Button/
      index.tsx
      Button.test.tsx
      Button.stories.tsx
      Button.spec.tsx
      (organized in folder)`}
              description="Grouping related files in a folder makes it easier to manage and maintain components."
            />
          </div>

        </div>
      </section>
    </div>
  )
}
