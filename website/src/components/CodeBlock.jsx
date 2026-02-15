import { Copy, Check } from 'lucide-react'
import { useState } from 'react'

export function CodeBlock({ code, language = 'javascript', title = null, showLineNumbers = false }) {
  const [copied, setCopied] = useState(false)

  const copyToClipboard = () => {
    navigator.clipboard.writeText(code)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const lines = code.split('\n')

  return (
    <div className="my-6 border border-gray-200 rounded-lg overflow-hidden bg-white">
      {title && (
        <div className="bg-gray-50 px-4 py-3 border-b border-gray-200 flex justify-between items-center">
          <span className="font-mono text-sm font-semibold text-gray-800">{title}</span>
        </div>
      )}
      <div className="relative group">
        <button
          onClick={copyToClipboard}
          className="absolute right-4 top-4 p-2 bg-white border border-gray-200 rounded hover:bg-gray-50 opacity-0 group-hover:opacity-100 transition-opacity z-10"
          title="Copy code"
        >
          {copied ? <Check className="w-4 h-4 text-green-600" /> : <Copy className="w-4 h-4" />}
        </button>
        <pre className="p-4 overflow-x-auto bg-gray-50">
          <code className="font-mono text-sm text-gray-800">
            {lines.map((line, i) => (
              <div key={i} className="flex">
                {showLineNumbers && (
                  <span className="inline-block w-8 text-gray-400 select-none pr-4 border-r border-gray-200 mr-4">
                    {i + 1}
                  </span>
                )}
                <span>{line}</span>
              </div>
            ))}
          </code>
        </pre>
      </div>
    </div>
  )
}

export function ExamplePair({ badCode, goodCode, description, language = 'javascript' }) {
  return (
    <div className="my-8 space-y-4">
      {description && <p className="text-gray-700">{description}</p>}
      <div className="grid md:grid-cols-2 gap-6">
        <div>
          <h4 className="text-lg font-semibold text-red-600 mb-3 flex items-center gap-2">
            ❌ BAD
          </h4>
          <CodeBlock code={badCode} language={language} />
        </div>
        <div>
          <h4 className="text-lg font-semibold text-green-600 mb-3 flex items-center gap-2">
            ✅ GOOD
          </h4>
          <CodeBlock code={goodCode} language={language} />
        </div>
      </div>
    </div>
  )
}
