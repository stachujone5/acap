import "./globals.css";
import { Nav } from "./nav";
import { Providers } from "./providers";

export default function RootLayout({ children }: { children: React.ReactNode }) {
	return (
		<html lang="en">
			<body className="relative flex h-screen flex-col overflow-y-hidden bg-background p-6 text-foreground">
				<Providers>
					<Nav />
					<main className="flex-grow overflow-y-hidden px-2 py-6">{children}</main>
				</Providers>
			</body>
		</html>
	);
}
