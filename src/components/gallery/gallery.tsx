import { mockGallery } from "@/data/mock-gallery"

export function Gallery() {
  return (
    <section
      aria-label="Photo Gallery"
      className="w-full"
    >
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
        {
            mockGallery.map((image) => (
                <div
                    key={image.id}
                    className="aspect-portrait overflow-hidden rounded-lg">
                        <img
                            src={image.src}
                            alt={image.alt}
                            loading="lazy"
                            className="h-full w-full object-cover">
                        </img>
                    </div>
        ))}
      </div>
    </section>
  )
}
