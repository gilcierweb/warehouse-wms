import { useRouter } from "vue-router";

// FlyonUI
import "flyonui/flyonui";

export default defineNuxtPlugin(() => {
  const router = useRouter();
  
  // Run on initial load
  if (typeof window !== 'undefined') {
    setTimeout(() => {
      if (window.HSStaticMethods) window.HSStaticMethods.autoInit();
      if (window.HSOverlay) window.HSOverlay.autoInit();
    }, 100);
  }
  
  // Run after each navigation
  router.afterEach(async () => {
    setTimeout(() => {
      if (window.HSStaticMethods) window.HSStaticMethods.autoInit();
      if (window.HSOverlay) window.HSOverlay.autoInit();
    }, 100);
  });
});