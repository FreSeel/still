import { useEffect, useState } from "react"

import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { GalleryTags } from "./gallery-tags"
import { GalleryFilter } from "./gallery-filter"
import { toast } from "@/components/ui/toast"

interface GalleryImage {
  url: string
  title: string
  tags: string[]
}

export function Gallery() {
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
        setError(null)
      } catch (err) {
        setError(err instanceof Error ? err.message : "Something went wrong")
      }
    }

    fetchImages()

    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:"
    const socket = new WebSocket(
      `${protocol}//${window.location.host}/api/ws/images`
    )

    socket.onmessage = (event) => {
      if (event.data === "image_uploaded") {
        fetchImages()
        toast.add({
          type: "info",
          description: "A new post was made",
        })
      }
    }

    return () => socket.close()
  }, [])

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
            <div className="relative aspect-3/4 w-full overflow-hidden rounded-lg">
              <DialogTrigger
                render={
                  <button type="button" className="block h-full w-full" />
                }
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
              </DialogTrigger>

              <GalleryTags tags={image.tags} />
            </div>

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
