"use client";

import * as NavigationMenuPrimitive from "@radix-ui/react-navigation-menu";
import NextLink from "next/link";
import { Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { Button } from "@/ui/button";
import { usePathname } from "next/navigation";
import { useMutation } from "@tanstack/react-query";
import { Theme, updateConfig } from "@/utils/bindings";

interface LinkProps {
	children: React.ReactNode;
	href: React.ComponentPropsWithoutRef<typeof NextLink>["href"];
}

const Link = ({ href, children }: LinkProps) => {
	const pathname = usePathname();
	const isActive = pathname === href;

	return (
		<NextLink href={href} passHref legacyBehavior>
			<NavigationMenuPrimitive.Link
				active={isActive}
				className="group inline-flex h-10 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-accent/50 data-[state=open]:bg-accent/50"
			>
				{children}
			</NavigationMenuPrimitive.Link>
		</NextLink>
	);
};

export const Nav = () => {
	const { setTheme } = useTheme();

	const { mutate: updateThemeConfig } = useMutation({
		mutationFn: (theme: Theme) => updateConfig({ theme }),
	});

	return (
		<div className="px-2">
			<NavigationMenuPrimitive.Root className="z-10 flex items-center justify-between border-b pb-2">
				<NavigationMenuPrimitive.List className="group flex flex-1 list-none items-center justify-center space-x-1">
					<NavigationMenuPrimitive.Item>
						<Link href="/">Recordings</Link>
					</NavigationMenuPrimitive.Item>
					<NavigationMenuPrimitive.Item>
						<Link href="/settings">Settings</Link>
					</NavigationMenuPrimitive.Item>
				</NavigationMenuPrimitive.List>

				<DropdownMenu>
					<DropdownMenuTrigger asChild>
						<Button variant="outline" size="icon">
							<Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
							<Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
						</Button>
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end">
						<DropdownMenuItem
							onClick={() => {
								setTheme("light");
								updateThemeConfig("light");
							}}
						>
							Light
						</DropdownMenuItem>
						<DropdownMenuItem
							onClick={() => {
								setTheme("dark");
								updateThemeConfig("dark");
							}}
						>
							Dark
						</DropdownMenuItem>
						<DropdownMenuItem
							onClick={() => {
								setTheme("system");
								updateThemeConfig("system");
							}}
						>
							System
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</NavigationMenuPrimitive.Root>
		</div>
	);
};
