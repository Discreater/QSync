import browser from './browser';

export function generateDeviceId() {
  const keys = [navigator.userAgent, new Date().getTime()];

  return btoa(keys.join('|')).replaceAll('=', '1');
}

export function getDeviceName() {
  let deviceName;
  if (browser.tizen)
    deviceName = 'Samsung Smart TV';

  else if (browser.web0s)
    deviceName = 'LG Smart TV';

  else if (browser.operaTv)
    deviceName = 'Opera TV';

  else if (browser.xboxOne)
    deviceName = 'Xbox One';

  else if (browser.ps4)
    deviceName = 'Sony PS4';

  else if (browser.chrome)
    deviceName = 'Chrome';

  else if (browser.edgeChromium)
    deviceName = 'Edge Chromium';

  else if (browser.edge)
    deviceName = 'Edge';

  else if (browser.firefox)
    deviceName = 'Firefox';

  else if (browser.opera)
    deviceName = 'Opera';

  else if (browser.safari)
    deviceName = 'Safari';

  else
    deviceName = 'Web Browser';

  if (browser.ipad)
    deviceName += ' iPad';

  else if (browser.iphone)
    deviceName += ' iPhone';

  else if (browser.android)
    deviceName += ' Android';

  return deviceName;
}
