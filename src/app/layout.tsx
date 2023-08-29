"use client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Nav } from "./nav";
import { ThemeProvider } from "next-themes";
import { useEffect } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import "./globals.css";

const queryClient = new QueryClient();

export default function RootLayout({ children }: { children: React.ReactNode }) {
	useEffect(() => {
		// Listener when window is not focused or in tray
		void listen("keypress", ({ payload: { message } }) => {
			console.log(message);
		});
	}, []);

	useEffect(() => {
		// Disable default events for F1-F12 keys and emit an event
		const handleKeyPress = (event: KeyboardEvent) => {
			if (event.key.startsWith("F") && !isNaN(parseInt(event.key.substring(1)))) {
				event.preventDefault();
				void emit("keypress", {
					message: event.key,
				});
			}
		};

		window.addEventListener("keydown", handleKeyPress);

		return () => {
			window.removeEventListener("keydown", handleKeyPress);
		};
	}, []);

	return (
		<html lang="en">
			<body className="relative flex h-screen flex-col overflow-y-hidden bg-background p-6 text-foreground">
				<ThemeProvider attribute="class" defaultTheme="system" enableSystem>
					<QueryClientProvider client={queryClient}>
						<Nav />
						<main className="flex-grow overflow-y-hidden px-2 py-6">{children}</main>
					</QueryClientProvider>
				</ThemeProvider>
			</body>
		</html>
	);
}
