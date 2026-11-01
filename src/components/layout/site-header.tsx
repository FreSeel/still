import { Moon, Plus } from "lucide-react"
import { BrandLogo } from "@/components/brand/brand-logo"
import { Button } from "@/components/ui/button"
import { UploadDialog } from "../upload/upload"
import { useState } from "react"

export function SiteHeader() {
  const [open, setOpen] = useState(false)

  return (
    <header className="border-b border-border/60">
      <div className="mx-auto flex h-18 max-w-7xl items-center justify-between px-6 lg:px-10">
        <BrandLogo />

        <div className="flex items-center gap-3">
          <Button
            type="button"
            variant="ghost"
            size="icon"
            disabled
            aria-label="Toggle theme (WIP)"
            className="size-12 shrink-0 rounded-full bg-black/25 text-foreground disabled:opacity-100"
          >
            <Moon className="size-5" aria-hidden="true" />
          </Button>

          <Button
            type="button"
            aria-label="Upload photo (WIP)"
            onClick={() => setOpen(true)}
            className="h-12 gap-2 rounded-full px-3 text-base font-medium disabled:opacity-100 sm:px-6"
          >
            <Plus className="size-5" aria-hidden="true" />
            <span className="hidden sm:inline">Upload</span>
          </Button>
          <UploadDialog open={open} onOpenChange={setOpen} />
        </div>
      </div>
    </header>
  )
}
