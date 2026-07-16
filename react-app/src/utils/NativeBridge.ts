export const NativeBridge = {
    /**
     * Check if the app is running inside a mobile WebView (Expo/Capacitor)
     */
    isMobile: () => {
        return !!(window as any).ReactNativeWebView;
    },

    /**
     * Send a message to the Native Shell
     */
    postMessage: (type: string, payload: any = {}) => {
        if (NativeBridge.isMobile()) {
            (window as any).ReactNativeWebView.postMessage(JSON.stringify({ type, ...payload }));
        }
    },

    /**
     * Trigger Native Haptics
     */
    hapticSuccess: () => NativeBridge.postMessage('HAPTIC_SUCCESS'),
    hapticError: () => NativeBridge.postMessage('HAPTIC_ERROR'),
    hapticImpact: () => NativeBridge.postMessage('HAPTIC_IMPACT'),
};
