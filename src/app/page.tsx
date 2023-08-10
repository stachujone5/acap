"use client";

import { Button } from "@/ui/button";
import { Toggle } from "@/ui/toggle";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";
import { Italic } from "lucide-react";
import { Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/ui/dropdown-menu";

const handleRecord = () => {
	return invoke("record_audio");
};

export default function Home() {
	const { mutate: startRecording } = useMutation({
		mutationFn: handleRecord,
	});

	return (
		<main className="relative flex h-full w-full flex-col items-center justify-center gap-5">
			<ModeToggle />
			<Button onClick={() => startRecording}>Record</Button>

			<Toggle variant="outline">
				<Italic className="h-4 w-4" />
			</Toggle>
		</main>
	);
}

const ModeToggle = () => {
	const { setTheme } = useTheme();

	return (
		<DropdownMenu>
			<DropdownMenuTrigger asChild>
				<Button variant="outline" size="icon" className="absolute right-0 top-0">
					<Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
					<Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
				</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end">
				<DropdownMenuItem onClick={() => setTheme("light")}>Light</DropdownMenuItem>
				<DropdownMenuItem onClick={() => setTheme("dark")}>Dark</DropdownMenuItem>
				<DropdownMenuItem onClick={() => setTheme("system")}>System</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
};
