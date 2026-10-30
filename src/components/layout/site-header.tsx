import { BrandLogo } from "@/components/brand/brand-logo";

export function SiteHeader() {
    return (
        <header className="border-b border-border/60">
            <div 
                className="
                    mx-auto flex h-[72px]
                    max-w-7xl items-center
                    px-6 lg:px-10
                    ">
                        <BrandLogo />
                    </div>
        </header>
    );
}