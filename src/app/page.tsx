"use client";

import { Button } from "@/ui/button";
import { recordAudio } from "@/utils/bindings";
import { useMutation } from "@tanstack/react-query";

const Home = () => {
	const { mutate: startRecording, isLoading } = useMutation({
		mutationFn: recordAudio,
	});

	return (
		<div>
			<Button disabled={isLoading} onClick={() => startRecording()}>
				Record
			</Button>
		</div>
	);
};

export default Home;
