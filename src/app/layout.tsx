import "./globals.css";
import { Providers } from "./providers";

export default function RootLayout({ children }: { children: React.ReactNode }) {
	return (
		<html lang="en">
			<body className="h-screen overflow-hidden p-4">
				<Providers>{children}</Providers>
			</body>
		</html>
	);
}
