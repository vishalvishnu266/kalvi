import React, { useRef } from 'react';
import { StyleSheet, View, SafeAreaView, Platform } from 'react-native';
import { WebView } from 'react-native-webview';
import * as Haptics from 'expo-haptics';
import { StatusBar } from 'expo-status-bar';

// Note: In production, this would be your deployed URL
const WEB_URL = 'http://192.168.1.XX:5173'; 

export default function App() {
  const webViewRef = useRef(null);

  const handleMessage = (event) => {
    try {
        const data = JSON.parse(event.nativeEvent.data);
        
        switch (data.type) {
          case 'HAPTIC_SUCCESS':
            Haptics.notificationAsync(Haptics.NotificationFeedbackType.Success);
            break;
          case 'HAPTIC_ERROR':
            Haptics.notificationAsync(Haptics.NotificationFeedbackType.Error);
            break;
          case 'HAPTIC_IMPACT':
            Haptics.impactAsync(Haptics.ImpactFeedbackStyle.Medium);
            break;
        }
    } catch (e) {
        console.warn('Bridge Error:', e);
    }
  };

  return (
    <SafeAreaView style={styles.container}>
      <StatusBar style="dark" />
      <View style={styles.webviewContainer}>
        <WebView
          ref={webViewRef}
          source={{ uri: WEB_URL }}
          onMessage={handleMessage}
          javaScriptEnabled={true}
          domStorageEnabled={true}
          startInLoadingState={true}
          allowsBackForwardNavigationGestures={true}
        />
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    paddingTop: Platform.OS === 'android' ? 25 : 0,
  },
  webviewContainer: {
    flex: 1,
  },
});
