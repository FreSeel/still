import { useId, useState } from "react"
import { Button } from "@/components/ui/button"

type GalleryTagsProps = {
  tags: string[]
}

export function GalleryTags({ tags }: GalleryTagsProps) {
  const [expanded, setExpanded] = useState(false)
  const tagsId = useId()

  if (!tags.length) return null

  return (
    <div className="group/tags absolute bottom-4 left-4 z-10 flex h-10 max-w-[calc(100%-2rem)] items-center overflow-hidden rounded-full border border-white/10 bg-zinc-950/30 text-white backdrop-blur-md">
      <Button
        type="button"
        variant="ghost"
        size="icon"
        aria-label="Toggle image tags"
        aria-expanded={expanded}
        aria-controls={tagsId}
        onClick={() => setExpanded((prev) => !prev)}
        className="size-10 shrink-0 rounded-full text-sm font-semibold text-white hover:bg-white/10 hover:text-white"
        onPointerEnter={(e) => {
          if (e.pointerType === "mouse") setExpanded(true)
        }}
        onPointerLeave={(e) => {
          if (e.pointerType === "mouse") setExpanded(false)
        }}
      >
        #
      </Button>
      <div
        className={`grid min-w-0 transition-[grid-template-columns] duration-300 ease-out ${expanded ? "grid-cols-[1fr]" : "grid-cols-[0fr]"}`}
      >
        <div className="min-w-0 overflow-hidden">
          <ul className="flex w-max flex-nowrap gap-1.5 py-2 pr-3">
            {tags.map((tag) => (
              <li
                key={tag}
                className="rounded-full bg-white/15 px-2.5 py-1 text-xs font-medium whitespace-nowrap"
              >
                {tag}
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  )
}
