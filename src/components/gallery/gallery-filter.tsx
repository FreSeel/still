import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

interface GalleryFilterProps {
  value: string
  onChange: (value: string) => void
}

export function GalleryFilter({ value, onChange }: GalleryFilterProps) {
  const [isOpen, setIsOpen] = useState(false)

  function handleToggle() {
    setIsOpen(!isOpen)

    if (isOpen) {
      onChange("")
    }
  }

  return (
    <div>
      <Button
        type="button"
        variant="ghost"
        onClick={handleToggle}
        aria-expanded={isOpen}
        aria-controls="gallery-tag-search"
        className="text-lg font-medium underline"
      >
        Explore
      </Button>

      {isOpen && (
        <Input
          id="gallery-tag-search"
          type="search"
          autoFocus
          placeholder="Type your tag.."
          value={value}
          onChange={(e) => onChange(e.target.value)}
          aria-label="Search gallery"
          className="max-w-sum w-full"
        />
      )}
    </div>
  )
}
