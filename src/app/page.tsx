"use client";

import { invoke } from "@tauri-apps/api/tauri";

export default function Home() {
	const handleRecord = () => {
		invoke("record_audio")
			.then(() => console.log("Audio recorded sucessfully"))
			.catch(() => console.log("Faild to record audio"));
	};

	return (
		<main className="flex h-full w-full flex-col items-center justify-center gap-5 bg-gradient-to-b from-[#2e026d] to-[#15162c]">
			<button
				onClick={handleRecord}
				className="rounded border border-gray-400 bg-white px-4 py-2 font-semibold text-gray-800 shadow transition-colors hover:bg-gray-200"
			>
				Record desktop audio
			</button>
		</main>
	);
}
