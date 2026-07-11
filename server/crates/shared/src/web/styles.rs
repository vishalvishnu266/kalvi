/// Tailwind style constants to keep HTML strings clean and readable.
/// Think of these like "CSS Classes" defined in Rust.

pub const INPUT: &str = "appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm";

pub const BTN_PRIMARY: &str = "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-[var(--primary-color)] hover:opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[var(--primary-color)] transition-all";

pub const CARD: &str = "bg-white dark:bg-gray-800 py-8 px-4 shadow sm:rounded-lg sm:px-10 border dark:border-gray-700 transition-colors";

pub const NAV_LINK: &str = "bg-[var(--primary-color)] text-white px-3 py-2 rounded-md text-sm font-medium opacity-90 hover:opacity-100 flex items-center transition-all";

pub const ERROR_BANNER: &str = "bg-red-50 dark:bg-red-900/30 border-l-4 border-red-400 p-4 mb-6";
