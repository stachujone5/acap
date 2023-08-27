"use client";

import { Button } from "@/ui/button";
import { Toggle } from "@/ui/toggle";
import { KeyboardEvent, ModifierKey, useEffect, useState } from "react";
import { dialog } from "@tauri-apps/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { updateConfigKey } from "@/utils/bindings";
import { CONFIG_QUERY_KEY, useConfig } from "@/utils/useConfig";
import { Input } from "@/ui/input";
import { Label } from "@/ui/label";
import { Skeleton } from "@/ui/skeleton";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";

const recordingDurationFormSchema = z.object({ duration: z.coerce.number().int().min(1).max(600) });

const changeSavePathFn = async () => {
	const pathSelectedByUser = await dialog.open({
		directory: true,
	});

	if (typeof pathSelectedByUser !== "string") {
		throw new Error("Invalid files / folders selected!");
	}

	return updateConfigKey({ savePath: pathSelectedByUser });
};

const changeRecordingDurationFn = (recordingDurationInSecs: number) => {
	return updateConfigKey({ recordingDurationInSecs });
};

const changeStartRecordingHotkeyFn = (hotkey: string) => {
	return updateConfigKey({ startRecordingHotkey: hotkey });
};

const Settings = () => {
	const {
		register,
		handleSubmit,
		reset,
		formState: { errors },
	} = useForm<z.infer<typeof recordingDurationFormSchema>>({
		resolver: zodResolver(recordingDurationFormSchema),
		mode: "onChange",
	});

	const [pressed, setPressed] = useState(false);

	const queryClient = useQueryClient();
	const { data: config } = useConfig();

	const { mutate: changeSavePathMutation } = useMutation({
		mutationFn: changeSavePathFn,
		onSuccess: (newConfig) => queryClient.setQueryData(CONFIG_QUERY_KEY, newConfig),
	});

	const { mutate: changeRecordingDurationMutation, isLoading: isRecordingDurationMutationLoading } =
		useMutation({
			mutationFn: changeRecordingDurationFn,
			onSuccess: (newConfig) => queryClient.setQueryData(CONFIG_QUERY_KEY, newConfig),
		});

	const { mutate: changeStartRecordingHotkeyMutation } = useMutation({
		mutationFn: changeStartRecordingHotkeyFn,
		onSuccess: (newConfig) => queryClient.setQueryData(CONFIG_QUERY_KEY, newConfig),
	});

	useEffect(() => {
		if (config) {
			reset({ duration: config.recordingDurationInSecs });
		}
	}, [config, reset]);

	const handleKeyDown = (e: KeyboardEvent<HTMLButtonElement>) => {
		e.preventDefault();
		setPressed(true);

		const modifierKeys: ModifierKey[] = [
			"Control",
			"Alt",
			"Shift",
			"Meta",
			"CapsLock",
			"AltGraph",
			"Fn",
		];

		const pressedModifier = modifierKeys.find((key) => e.getModifierState(key));

		if (!pressedModifier) {
			return;
		}

		if (e.key !== pressedModifier && e.code !== "Space") {
			changeStartRecordingHotkeyMutation(`${pressedModifier} + ${e.key}`);
		}
	};

	const onRecordingDurationFormSubmit = (data: z.infer<typeof recordingDurationFormSchema>) => {
		changeRecordingDurationMutation(data.duration);
	};

	return (
		<div className="flex flex-col gap-6">
			<div className="flex items-center gap-4">
				<p className="text-xl font-semibold tracking-tight">Start recording hotkey</p>
				{config ? (
					<Toggle
						pressed={pressed}
						onFocus={() => setPressed(true)}
						onBlur={() => setPressed(false)}
						variant="outline"
						onKeyDown={handleKeyDown}
						onKeyUp={() => setPressed(false)}
					>
						<div className="flex items-center justify-center">{config.startRecordingHotkey}</div>
					</Toggle>
				) : (
					<Skeleton className="h-10 w-20" />
				)}

				{pressed && <p>Listening for keyboard input</p>}
			</div>

			<form
				className="flex h-10 items-center gap-4"
				onSubmit={handleSubmit(onRecordingDurationFormSubmit)}
			>
				<Label htmlFor="recording_duration">Recording duration in seconds</Label>
				{config ? (
					<Input
						defaultValue={config.recordingDurationInSecs.toString()}
						id="recording_duration"
						className="w-20"
						{...register("duration")}
					/>
				) : (
					<Skeleton className="h-full w-20" />
				)}
				<Button
					disabled={isRecordingDurationMutationLoading || Boolean(errors.duration)}
					type="submit"
				>
					Save
				</Button>
				{errors.duration && <p className="text-sm">Expected an integer between 1 and 600</p>}
			</form>

			<div className="flex h-10 items-center gap-4">
				<p className="text-xl font-semibold tracking-tight">Save directory</p>

				{config ? (
					<p className="inline-flex h-10 items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium">
						{config.savePath}
					</p>
				) : (
					<Skeleton className="h-full w-1/3" />
				)}

				<Button disabled={!Boolean(config)} onClick={() => changeSavePathMutation()}>
					Change
				</Button>
			</div>
		</div>
	);
};

export default Settings;
