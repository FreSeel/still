import { SiteHeader } from "@/components/layout/site-header"
import { Gallery } from "@/components/gallery/gallery"
import { SiteFooter } from "@/components/layout/site-footer"
import { Toaster } from "@/components/ui/toast"

export function App() {
  return (
    <div className="min-h-svh bg-background text-foreground">
      <SiteHeader />
      <main className="mx-auto w-full max-w-7xl flex-1 px-6 py-12 sm:py-16 lg:px-10">
        <section className="mb-12">
          <h1 className="text-3x1 font-semibold tracking-tight sm:text-4xl">
            Moments worth sharing.
          </h1>
          <p className="mt-3 text-base text-muted-foreground">
            A collection of moments. Shared anonymously.
          </p>
        </section>
        <Gallery />
        <Toaster />
      </main>

      <SiteFooter />
    </div>
  )
}

export default App
