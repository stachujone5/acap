"use client";

import { Button } from "@/ui/button";
import { recordAudio } from "@/utils/bindings";
import { useMutation } from "@tanstack/react-query";

const Home = () => {
	const { mutate: startRecording } = useMutation({
		mutationFn: recordAudio,
	});

	return (
		<div>
			<Button onClick={() => startRecording()}>Record</Button>
		</div>
	);
};

export default Home;
