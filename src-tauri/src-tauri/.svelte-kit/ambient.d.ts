
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```sh
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const NVM_INC: string;
	export const NoDefaultCurrentDirectoryInExePath: string;
	export const CLAUDE_CODE_ENTRYPOINT: string;
	export const TERM_PROGRAM: string;
	export const NODE: string;
	export const AZURE_API_KEY: string;
	export const NVM_CD_FLAGS: string;
	export const SHELL: string;
	export const TERM: string;
	export const WARP_HONOR_PS1: string;
	export const HOMEBREW_API_DOMAIN: string;
	export const HOMEBREW_BOTTLE_DOMAIN: string;
	export const TMPDIR: string;
	export const HOMEBREW_REPOSITORY: string;
	export const CONDA_SHLVL: string;
	export const TERM_PROGRAM_VERSION: string;
	export const FPATH: string;
	export const npm_config_local_prefix: string;
	export const PNPM_HOME: string;
	export const GIT_EDITOR: string;
	export const USER: string;
	export const NVM_DIR: string;
	export const COMMAND_MODE: string;
	export const CONDA_EXE: string;
	export const HOMEBREW_CORE_GIT_REMOTE: string;
	export const SSH_AUTH_SOCK: string;
	export const WARP_IS_LOCAL_SHELL_SESSION: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const PUB_HOSTED_URL: string;
	export const npm_execpath: string;
	export const HOMEBREW_PIP_INDEX_URL: string;
	export const WARP_USE_SSH_WRAPPER: string;
	export const _CE_CONDA: string;
	export const PATH: string;
	export const npm_package_json: string;
	export const _: string;
	export const LaunchInstanceID: string;
	export const __CFBundleIdentifier: string;
	export const npm_command: string;
	export const PWD: string;
	export const CONTEXT7_API_KEY: string;
	export const FLUTTER_STORAGE_BASE_URL: string;
	export const OPENROUTER_API_KEY: string;
	export const JAVA_HOME: string;
	export const npm_lifecycle_event: string;
	export const OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE: string;
	export const npm_package_name: string;
	export const NODE_PATH: string;
	export const XPC_FLAGS: string;
	export const npm_package_version: string;
	export const XPC_SERVICE_NAME: string;
	export const AZURE_OPENAI_API_KEY: string;
	export const _CE_M: string;
	export const GEMINI_API_KEY: string;
	export const HOME: string;
	export const SHLVL: string;
	export const HOMEBREW_BREW_GIT_REMOTE: string;
	export const HOMEBREW_PREFIX: string;
	export const LOGNAME: string;
	export const CONDA_PYTHON_EXE: string;
	export const npm_lifecycle_script: string;
	export const LC_CTYPE: string;
	export const COREPACK_ENABLE_AUTO_PIN: string;
	export const SSH_SOCKET_DIR: string;
	export const NVM_BIN: string;
	export const BUN_INSTALL: string;
	export const npm_config_user_agent: string;
	export const HOMEBREW_CELLAR: string;
	export const INFOPATH: string;
	export const HF_TOKEN: string;
	export const CONDA_CHANGEPS1: string;
	export const CLAUDECODE: string;
	export const SECURITYSESSIONID: string;
	export const npm_node_execpath: string;
	export const COLORTERM: string;
	export const TEST: string;
	export const VITEST: string;
	export const NODE_ENV: string;
	export const PROD: string;
	export const DEV: string;
	export const BASE_URL: string;
	export const MODE: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		NVM_INC: string;
		NoDefaultCurrentDirectoryInExePath: string;
		CLAUDE_CODE_ENTRYPOINT: string;
		TERM_PROGRAM: string;
		NODE: string;
		AZURE_API_KEY: string;
		NVM_CD_FLAGS: string;
		SHELL: string;
		TERM: string;
		WARP_HONOR_PS1: string;
		HOMEBREW_API_DOMAIN: string;
		HOMEBREW_BOTTLE_DOMAIN: string;
		TMPDIR: string;
		HOMEBREW_REPOSITORY: string;
		CONDA_SHLVL: string;
		TERM_PROGRAM_VERSION: string;
		FPATH: string;
		npm_config_local_prefix: string;
		PNPM_HOME: string;
		GIT_EDITOR: string;
		USER: string;
		NVM_DIR: string;
		COMMAND_MODE: string;
		CONDA_EXE: string;
		HOMEBREW_CORE_GIT_REMOTE: string;
		SSH_AUTH_SOCK: string;
		WARP_IS_LOCAL_SHELL_SESSION: string;
		__CF_USER_TEXT_ENCODING: string;
		PUB_HOSTED_URL: string;
		npm_execpath: string;
		HOMEBREW_PIP_INDEX_URL: string;
		WARP_USE_SSH_WRAPPER: string;
		_CE_CONDA: string;
		PATH: string;
		npm_package_json: string;
		_: string;
		LaunchInstanceID: string;
		__CFBundleIdentifier: string;
		npm_command: string;
		PWD: string;
		CONTEXT7_API_KEY: string;
		FLUTTER_STORAGE_BASE_URL: string;
		OPENROUTER_API_KEY: string;
		JAVA_HOME: string;
		npm_lifecycle_event: string;
		OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE: string;
		npm_package_name: string;
		NODE_PATH: string;
		XPC_FLAGS: string;
		npm_package_version: string;
		XPC_SERVICE_NAME: string;
		AZURE_OPENAI_API_KEY: string;
		_CE_M: string;
		GEMINI_API_KEY: string;
		HOME: string;
		SHLVL: string;
		HOMEBREW_BREW_GIT_REMOTE: string;
		HOMEBREW_PREFIX: string;
		LOGNAME: string;
		CONDA_PYTHON_EXE: string;
		npm_lifecycle_script: string;
		LC_CTYPE: string;
		COREPACK_ENABLE_AUTO_PIN: string;
		SSH_SOCKET_DIR: string;
		NVM_BIN: string;
		BUN_INSTALL: string;
		npm_config_user_agent: string;
		HOMEBREW_CELLAR: string;
		INFOPATH: string;
		HF_TOKEN: string;
		CONDA_CHANGEPS1: string;
		CLAUDECODE: string;
		SECURITYSESSIONID: string;
		npm_node_execpath: string;
		COLORTERM: string;
		TEST: string;
		VITEST: string;
		NODE_ENV: string;
		PROD: string;
		DEV: string;
		BASE_URL: string;
		MODE: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
