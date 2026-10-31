import { useState } from "react"
import { mockGallery } from "@/data/mock-gallery"

import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { GalleryTags } from "./gallery-tags"
import { GalleryFilter } from "./gallery-filter"

export function Gallery() {
  const [tagQuery, setTagQuery] = useState("")

  const filteredGallery = mockGallery.filter((image) => {
    const query = tagQuery.trim().toLowerCase()

    if (!query) return true
    // ToDo: Think about filtering via backend instead...
    // Pros: Performance
    // Cons: more requests
    // Decisions decisions...
    return image.tags.some((tag) => tag.toLowerCase().includes(query))
  })

  return (
    <section aria-label="Photo Gallery" className="w-full">
      <GalleryFilter value={tagQuery} onChange={setTagQuery}></GalleryFilter>
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
        {filteredGallery.map((image) => (
          <Dialog key={image.id}>
            <DialogTrigger asChild>
              <button
                type="button"
                className="relative block aspect-3/4 w-full overflow-hidden rounded-lg"
              >
                <img
                  src={image.src}
                  alt={image.alt}
                  loading="lazy"
                  className="h-full w-full object-cover"
                />
                {/* Title pill */}
                <span className="absolute top-4 left-1/2 z-10 -translate-x-1/2 rounded-full bg-zinc-900/30 px-5 py-2 text-sm font-medium text-white backdrop-blur-sm">
                  {image.alt}
                </span>
                <GalleryTags tags={image.tags} />
              </button>
            </DialogTrigger>

            <DialogContent className="w-[95vw] sm:max-w-5xl">
              <DialogTitle className="">{image.alt}</DialogTitle>

              <img
                src={image.src}
                alt={image.alt}
                className="max-h-[80vh] w-full object-contain"
              />
            </DialogContent>
          </Dialog>
        ))}
      </div>
    </section>
  )
}
