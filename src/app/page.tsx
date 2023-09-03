"use client";

import { Skeleton } from "@/ui/skeleton";
import { getAcapFiles } from "@/utils/bindings";
import { useConfig } from "@/utils/useConfig";
import { useQuery } from "@tanstack/react-query";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { useEffect } from "react";
import { useTheme } from "next-themes";

const getRecordings = async () => {
	const acapFiles = await getAcapFiles();

	const convertedRecordings = acapFiles.map((r) => {
		return {
			path: convertFileSrc(r.path),
			name: r.name,
		};
	});

	return convertedRecordings;
};

const RecordingsPage = () => {
	const { setTheme } = useTheme();
	const { data: config } = useConfig();
	const { data: recordings } = useQuery({
		queryKey: ["recordings"],
		queryFn: getRecordings,
	});

	useEffect(() => {
		if (config) {
			setTheme(config.theme);
		}
	}, [config, setTheme]);

	if (recordings?.length === 0) {
		return (
			<div className="flex h-full flex-col items-center justify-center gap-4">
				<p className="text-2xl font-semibold tracking-tight">No recordings found!</p>
			</div>
		);
	}

	return (
		<div className="flex h-full flex-col gap-4 overflow-y-auto">
			{recordings ? (
				recordings.map((recording) => (
					<div
						key={recording.path}
						className="flex h-[100px] items-center gap-4 rounded-md border px-4 py-6"
					>
						<p>{recording.name}</p>
						<audio controls src={recording.path} />
					</div>
				))
			) : (
				<>
					<Skeleton className="h-[100px]" />
					<Skeleton className="h-[100px]" />
					<Skeleton className="h-[100px]" />
					<Skeleton className="h-[100px]" />
				</>
			)}
		</div>
	);
};

export default RecordingsPage;
