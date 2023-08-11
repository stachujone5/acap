"use client";

import { useQuery } from "@tanstack/react-query";
import { getConfig } from "./bindings";

export const CONFIG_QUERY_KEY = ["config"];

export const useConfig = () => {
	return useQuery({
		queryKey: CONFIG_QUERY_KEY,
		queryFn: getConfig,
	});
};
