
import { mockGallery } from "@/data/mock-gallery"

import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"

export function Gallery() {
  return (
    <section aria-label="Photo Gallery" className="w-full">
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
        {mockGallery.map((image) => (
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
              </button>
            </DialogTrigger>

            <DialogContent className="w-[95vw] sm:max-w-5xl">
              <DialogTitle className="">
                {image.alt}
              </DialogTitle>

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
