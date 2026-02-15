import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { Navigation } from './components/Navigation'
import { Footer } from './components/Footer'
import Home from './pages/Home'
import GettingStartedPage from './pages/GettingStartedPage'
import RulesPage from './pages/RulesPage'
import ConfigurationPage from './pages/ConfigurationPage'
import SchemasPage from './pages/SchemasPage'

export default function App() {
  return (
    <Router>
      <div className="min-h-screen bg-white text-gray-900 flex flex-col">
        <Navigation />
        <main className="flex-grow">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/docs/getting-started" element={<GettingStartedPage />} />
            <Route path="/docs/rules" element={<RulesPage />} />
            <Route path="/docs/configuration" element={<ConfigurationPage />} />
            <Route path="/docs/schemas" element={<SchemasPage />} />
          </Routes>
        </main>
        <Footer />
      </div>
    </Router>
  )
}

