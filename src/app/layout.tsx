import "./globals.css";
import { Nav } from "./nav";
import { Providers } from "./providers";

export default function RootLayout({ children }: { children: React.ReactNode }) {
	return (
		<html lang="en">
			<body className="relative h-screen overflow-hidden bg-background p-6 text-foreground">
				<Providers>
					<Nav />
					<main className="h-full w-full py-6">{children}</main>
				</Providers>
			</body>
		</html>
	);
}
