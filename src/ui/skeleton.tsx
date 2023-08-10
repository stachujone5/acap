import { twMerge } from "tailwind-merge";

const Skeleton = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => {
	return <div className={twMerge("animate-pulse rounded-md bg-muted", className)} {...props} />;
};

export { Skeleton };
