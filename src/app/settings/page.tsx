"use client";

import { Button } from "@/ui/button";
import { Toggle } from "@/ui/toggle";
import { useEffect, useState } from "react";
import { dialog } from "@tauri-apps/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { updateConfigKey } from "@/utils/bindings";
import { CONFIG_QUERY_KEY, useConfig } from "@/utils/useConfig";
import { Input } from "@/ui/input";
import { Label } from "@/ui/label";
import { z } from "zod";

const recordingDurationSchema = z.coerce.number().int().min(1).max(600);

const changeSavePathMutation = async () => {
	const pathSelectedByUser = await dialog.open({
		directory: true,
	});

	if (typeof pathSelectedByUser !== "string") {
		throw new Error("Invalid files / folders selected!");
	}

	return updateConfigKey({ savePath: pathSelectedByUser });
};

const changeRecordingDurationMutation = (recordingDurationInSecs: number) => {
	return updateConfigKey({ recordingDurationInSecs });
};

const Settings = () => {
	const [pressed, setPressed] = useState(false);
	const [key, setKey] = useState("F10");
	const [recordingDurationInputValue, setRecordingDurationInputValue] = useState("");
	const isRecordingDurationInputValueCorrect = recordingDurationSchema.safeParse(
		recordingDurationInputValue,
	).success;

	const queryClient = useQueryClient();
	const { data: config, isLoading: isConfigLoading, isError: isConfigError } = useConfig();

	const { mutate: changeSavePath } = useMutation({
		mutationFn: changeSavePathMutation,
		onSuccess: (newConfig) => queryClient.setQueryData(CONFIG_QUERY_KEY, newConfig),
	});

	const { mutate: changeRecordingDuration, isLoading: isRecordingDurationLoading } = useMutation({
		mutationFn: changeRecordingDurationMutation,
		onSuccess: (newConfig) => queryClient.setQueryData(CONFIG_QUERY_KEY, newConfig),
	});

	useEffect(() => {
		if (config) {
			setRecordingDurationInputValue(config.recordingDurationInSecs.toString());
		}
	}, [config]);

	const handleFormSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		const parsedValue = recordingDurationSchema.safeParse(recordingDurationInputValue);

		if (parsedValue.success) {
			changeRecordingDuration(parsedValue.data);
		}

		e.preventDefault();
	};

	const handleKeyDown = (e: React.KeyboardEvent<HTMLButtonElement>) => {
		setKey(e.key);
	};

	if (!config) {
		return null;
	}

	return (
		<div className="flex flex-col gap-6">
			<div className="flex items-center gap-4">
				<p className="text-xl font-semibold tracking-tight">Start recording hotkey</p>
				<Toggle
					pressed={pressed}
					onPressedChange={(isPressed) => setPressed(isPressed)}
					variant="outline"
					onKeyDown={handleKeyDown}
				>
					<div className="flex items-center justify-center">{key}</div>
				</Toggle>

				{pressed && <p>Listening for keyboard input</p>}
			</div>

			<form className="flex h-10 items-center gap-4" onSubmit={handleFormSubmit}>
				<Label htmlFor="recording_duration">Recording duration in seconds</Label>
				<Input
					onChange={(e) => setRecordingDurationInputValue(e.currentTarget.value)}
					value={recordingDurationInputValue}
					id="recording_duration"
					className="w-20"
				/>

				<Button
					disabled={isRecordingDurationLoading || !isRecordingDurationInputValueCorrect}
					type="submit"
				>
					Save
				</Button>
				{!isRecordingDurationInputValueCorrect && (
					<p className="text-sm">Value must be integer between 1 and 600</p>
				)}
			</form>

			<div className="flex h-10 items-center gap-4">
				<p className="text-xl font-semibold tracking-tight">Save directory</p>

				<p className="inline-flex h-10 items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium">
					{config.savePath}
				</p>

				<Button disabled={!Boolean(config)} onClick={() => changeSavePath()}>
					Change
				</Button>
			</div>
		</div>
	);
};

export default Settings;
