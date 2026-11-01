import { useEffect, useState } from "react"

import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { GalleryTags } from "./gallery-tags"
import { GalleryFilter } from "./gallery-filter"

interface GalleryImage {
  url: string
  title: string
  tags: string[]
}

interface GalleryProps {
  refreshKey: number
}

export function Gallery({ refreshKey }: GalleryProps) {
  const [images, setImages] = useState<GalleryImage[]>([])
  const [tagQuery, setTagQuery] = useState("")
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchImages = async () => {
      try {
        const response = await fetch("/api/images")

        if (!response.ok) {
          throw new Error("Failed to fetch images")
        }

        const data: GalleryImage[] = await response.json()
        setImages(data)
      } catch (err) {
        setError(err instanceof Error ? err.message : "Something went wrong")
      }
    }

    fetchImages()
  }, [refreshKey])

  const filteredGallery = images.filter((image) => {
    const query = tagQuery.trim().toLowerCase()

    if (!query) return true

    return image.tags.some((tag) => tag.toLowerCase().includes(query))
  })

  return (
    <section aria-label="Photo Gallery" className="w-full">
      <GalleryFilter value={tagQuery} onChange={setTagQuery} />

      {error && <p className="text-sm text-destructive">{error}</p>}

      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
        {filteredGallery.map((image) => (
          <Dialog key={image.url}>
            <DialogTrigger asChild>
              <button
                type="button"
                className="relative block aspect-3/4 w-full overflow-hidden rounded-lg"
              >
                <img
                  src={image.url}
                  alt={image.title}
                  loading="lazy"
                  className="h-full w-full object-cover"
                />

                <span className="absolute top-4 left-1/2 z-10 -translate-x-1/2 rounded-full bg-zinc-900/30 px-5 py-2 text-sm font-medium text-white backdrop-blur-sm">
                  {image.title}
                </span>

                <GalleryTags tags={image.tags} />
              </button>
            </DialogTrigger>

            <DialogContent className="w-[95vw] sm:max-w-5xl">
              <DialogTitle>{image.title}</DialogTitle>

              <img
                src={image.url}
                alt={image.title}
                className="max-h-[80vh] w-full object-contain"
              />
            </DialogContent>
          </Dialog>
        ))}
      </div>
    </section>
  )
}
