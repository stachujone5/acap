"use client";

import { Button } from "@/ui/button";
import { Skeleton } from "@/ui/skeleton";
import { getAcapFiles, recordAudio } from "@/utils/bindings";
import { useConfig } from "@/utils/useConfig";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import * as ProgressPrimitive from "@radix-ui/react-progress";

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
	const queryClient = useQueryClient();
	const { data: config } = useConfig();
	const { data: recordings } = useQuery({
		queryKey: ["recordings"],
		queryFn: getRecordings,
	});

	const { mutate: startRecording, isLoading } = useMutation({
		mutationFn: recordAudio,
		onSuccess: () => queryClient.invalidateQueries(["recordings"]),
	});

	if (recordings && recordings.length === 0 && !isLoading) {
		return (
			<div className="flex h-full flex-col items-center justify-center gap-4">
				<p className="text-2xl font-semibold tracking-tight">No recordings found!</p>
				<Button disabled={isLoading} onClick={() => startRecording()}>
					Record something
				</Button>
			</div>
		);
	}

	return (
		<div className="flex h-full flex-col gap-4">
			<div className="flex items-center gap-4">
				{config ? (
					<Button className="h-10 w-40" disabled={isLoading} onClick={() => startRecording()}>
						Record new audio
					</Button>
				) : (
					<Skeleton className="h-10 w-40" />
				)}
			</div>
			<div className="flex flex-grow flex-col gap-4 overflow-y-auto">
				{isLoading && config && (
					<div>
						<ProgressPrimitive.Root className="relative h-[100px] overflow-hidden rounded-md border">
							<ProgressPrimitive.Indicator
								className="progress-animation h-full w-full flex-1 animate-pulse rounded-md bg-muted transition-all"
								style={{ animationDuration: `${config.recording_duration_in_secs}s` }}
							/>
						</ProgressPrimitive.Root>
					</div>
				)}
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
		</div>
	);
};

export default Recordings;
