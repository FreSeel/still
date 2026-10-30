import { SiteHeader } from "@/components/layout/site-header"

export function App() {
  return (
    <div className="min-h-svh bg-background text-foreground">
      <SiteHeader />
      <main className="mx-auto max-w-7xl px-6 py-10">
        {/* ToDo: Add Gallery component and footer */}
      </main>
    </div>
  )
}

export default App
