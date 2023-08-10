"use client";

import { Skeleton } from "@/ui/skeleton";
import { getAcapFiles } from "@/utils/bindings";
import { useQuery } from "@tanstack/react-query";

import { convertFileSrc } from "@tauri-apps/api/tauri";

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

const Recordings = () => {
	const { data } = useQuery({
		queryKey: ["recordings"],
		queryFn: getRecordings,
	});

	return (
		<div className="flex h-full flex-col gap-4 overflow-y-auto">
			{data ? (
				data?.map((recording) => (
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

export default Recordings;
