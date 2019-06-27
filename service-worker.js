/**
 * Copyright 2016 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

// DO NOT EDIT THIS GENERATED OUTPUT DIRECTLY!
// This file should be overwritten as part of your build process.
// If you need to extend the behavior of the generated service worker, the best approach is to write
// additional code and include it using the importScripts option:
//   https://github.com/GoogleChrome/sw-precache#importscripts-arraystring
//
// Alternatively, it's possible to make changes to the underlying template file and then use that as the
// new base for generating output, via the templateFilePath option:
//   https://github.com/GoogleChrome/sw-precache#templatefilepath-string
//
// If you go that route, make sure that whenever you update your sw-precache dependency, you reconcile any
// changes made to this original template file with your modified copy.

// This generated service worker JavaScript will precache your site's resources.
// The code needs to be saved in a .js file at the top-level of your site, and registered
// from your pages in order to be used. See
// https://github.com/googlechrome/sw-precache/blob/master/demo/app/js/service-worker-registration.js
// for an example of how you can register this script and handle various service worker events.

/* eslint-env worker, serviceworker */
/* eslint-disable indent, no-unused-vars, no-multiple-empty-lines, max-nested-callbacks, space-before-function-paren, quotes, comma-spacing */
'use strict';

var precacheConfig = [["1.bundle.js","03b1a2c9af535bd51da22730023f3f45"],["140948bb3ebb27033f3ed626a87cc485.png","140948bb3ebb27033f3ed626a87cc485"],["23f8548f2a79bdfbf5c75d567167ae82.png","23f8548f2a79bdfbf5c75d567167ae82"],["257ddeaa56614f57476f17fe845e4cfa.png","257ddeaa56614f57476f17fe845e4cfa"],["28c483710cff64d10e2f959490227081.png","28c483710cff64d10e2f959490227081"],["28d9191ddfef0382fd21cc712cc6dee9.png","28d9191ddfef0382fd21cc712cc6dee9"],["29120fc1cb5592377dd92daf2825faf0.png","29120fc1cb5592377dd92daf2825faf0"],["2ed09b57e356edad8d9cdef4218babe7.png","2ed09b57e356edad8d9cdef4218babe7"],["2f3b1be627a28b780321e09343274106.png","2f3b1be627a28b780321e09343274106"],["2fc028ad14099e84fffc65b5c1d1a56c.png","2fc028ad14099e84fffc65b5c1d1a56c"],["33726fbcb7dd80e928f64c19f6b77381.png","33726fbcb7dd80e928f64c19f6b77381"],["426f6d43f5528bc2c7f1e112609c90c5.png","426f6d43f5528bc2c7f1e112609c90c5"],["46e20d86d16e1aa9b62e718d08147eef.png","46e20d86d16e1aa9b62e718d08147eef"],["545e9144c338a09d0964fe686cb1fdf0.png","545e9144c338a09d0964fe686cb1fdf0"],["598356ddf665c325e46453ed3ac2449d.png","598356ddf665c325e46453ed3ac2449d"],["5c41cffad63a5b67a3072fd52273218d.png","5c41cffad63a5b67a3072fd52273218d"],["6756022bbef7a298400f8a244f778919.png","6756022bbef7a298400f8a244f778919"],["6dce7f3372a412719d738f3f89b9e6eb.png","6dce7f3372a412719d738f3f89b9e6eb"],["7329f3b81c7bab3d555f1945e0585fcd.png","7329f3b81c7bab3d555f1945e0585fcd"],["812b2f5d5eb73b91442b055527b4b1cf.png","812b2f5d5eb73b91442b055527b4b1cf"],["8a030ae90ecc7b074af4eeba6de5fad9.png","8a030ae90ecc7b074af4eeba6de5fad9"],["8e28dae4b0729f881e7039cd45f66508.png","8e28dae4b0729f881e7039cd45f66508"],["95abe119d6b03eeee48562e7e028a6a4.png","95abe119d6b03eeee48562e7e028a6a4"],["9e27cf6edbe1d107a385d0f7046124ca.png","9e27cf6edbe1d107a385d0f7046124ca"],["a6567a75cd44a18e8a431f57014b4948.png","a6567a75cd44a18e8a431f57014b4948"],["b43550c7072049581379897a765026a8.png","b43550c7072049581379897a765026a8"],["bundle.js","b933182c7febefb54ee0b008d7c1dab5"],["c18823d9bf72ff812ca449cb30547723.png","c18823d9bf72ff812ca449cb30547723"],["c18eadb805abd3e802803c4423aec15c.png","c18eadb805abd3e802803c4423aec15c"],["cd70c5d54494f3b652628c9edf7625c7.png","cd70c5d54494f3b652628c9edf7625c7"],["cf3c28931ef1cb4f926a480dab498794.png","cf3c28931ef1cb4f926a480dab498794"],["d7fc054174122b3227838a30243d994b.png","d7fc054174122b3227838a30243d994b"],["e0d81185ba8de5063c0a.module.wasm","5e2d90ebe857488e351867d27d818cb0"],["f4f93e1240aaadb3afc04541a2cb579e.png","f4f93e1240aaadb3afc04541a2cb579e"],["favicon.ico","fc0da992b2958547d789da8b1ca26f55"],["icon-192.png","3169651b01fae629b50216b0e4610c64"],["icon-512.png","e186fb35d5ef78477305638633b1a1e6"],["index.html","f70482d12d932822e21abf27cfa2802e"],["manifest.json","5db63a56e1e183fea3f525ea810e21c3"]];
var cacheName = 'sw-precache-v3-sw-precache-webpack-plugin-' + (self.registration ? self.registration.scope : '');


var ignoreUrlParametersMatching = [/^utm_/];



var addDirectoryIndex = function(originalUrl, index) {
    var url = new URL(originalUrl);
    if (url.pathname.slice(-1) === '/') {
      url.pathname += index;
    }
    return url.toString();
  };

var cleanResponse = function(originalResponse) {
    // If this is not a redirected response, then we don't have to do anything.
    if (!originalResponse.redirected) {
      return Promise.resolve(originalResponse);
    }

    // Firefox 50 and below doesn't support the Response.body stream, so we may
    // need to read the entire body to memory as a Blob.
    var bodyPromise = 'body' in originalResponse ?
      Promise.resolve(originalResponse.body) :
      originalResponse.blob();

    return bodyPromise.then(function(body) {
      // new Response() is happy when passed either a stream or a Blob.
      return new Response(body, {
        headers: originalResponse.headers,
        status: originalResponse.status,
        statusText: originalResponse.statusText
      });
    });
  };

var createCacheKey = function(originalUrl, paramName, paramValue,
                           dontCacheBustUrlsMatching) {
    // Create a new URL object to avoid modifying originalUrl.
    var url = new URL(originalUrl);

    // If dontCacheBustUrlsMatching is not set, or if we don't have a match,
    // then add in the extra cache-busting URL parameter.
    if (!dontCacheBustUrlsMatching ||
        !(url.pathname.match(dontCacheBustUrlsMatching))) {
      url.search += (url.search ? '&' : '') +
        encodeURIComponent(paramName) + '=' + encodeURIComponent(paramValue);
    }

    return url.toString();
  };

var isPathWhitelisted = function(whitelist, absoluteUrlString) {
    // If the whitelist is empty, then consider all URLs to be whitelisted.
    if (whitelist.length === 0) {
      return true;
    }

    // Otherwise compare each path regex to the path of the URL passed in.
    var path = (new URL(absoluteUrlString)).pathname;
    return whitelist.some(function(whitelistedPathRegex) {
      return path.match(whitelistedPathRegex);
    });
  };

var stripIgnoredUrlParameters = function(originalUrl,
    ignoreUrlParametersMatching) {
    var url = new URL(originalUrl);
    // Remove the hash; see https://github.com/GoogleChrome/sw-precache/issues/290
    url.hash = '';

    url.search = url.search.slice(1) // Exclude initial '?'
      .split('&') // Split into an array of 'key=value' strings
      .map(function(kv) {
        return kv.split('='); // Split each 'key=value' string into a [key, value] array
      })
      .filter(function(kv) {
        return ignoreUrlParametersMatching.every(function(ignoredRegex) {
          return !ignoredRegex.test(kv[0]); // Return true iff the key doesn't match any of the regexes.
        });
      })
      .map(function(kv) {
        return kv.join('='); // Join each [key, value] array into a 'key=value' string
      })
      .join('&'); // Join the array of 'key=value' strings into a string with '&' in between each

    return url.toString();
  };


var hashParamName = '_sw-precache';
var urlsToCacheKeys = new Map(
  precacheConfig.map(function(item) {
    var relativeUrl = item[0];
    var hash = item[1];
    var absoluteUrl = new URL(relativeUrl, self.location);
    var cacheKey = createCacheKey(absoluteUrl, hashParamName, hash, false);
    return [absoluteUrl.toString(), cacheKey];
  })
);

function setOfCachedUrls(cache) {
  return cache.keys().then(function(requests) {
    return requests.map(function(request) {
      return request.url;
    });
  }).then(function(urls) {
    return new Set(urls);
  });
}

self.addEventListener('install', function(event) {
  event.waitUntil(
    caches.open(cacheName).then(function(cache) {
      return setOfCachedUrls(cache).then(function(cachedUrls) {
        return Promise.all(
          Array.from(urlsToCacheKeys.values()).map(function(cacheKey) {
            // If we don't have a key matching url in the cache already, add it.
            if (!cachedUrls.has(cacheKey)) {
              var request = new Request(cacheKey, {credentials: 'same-origin'});
              return fetch(request).then(function(response) {
                // Bail out of installation unless we get back a 200 OK for
                // every request.
                if (!response.ok) {
                  throw new Error('Request for ' + cacheKey + ' returned a ' +
                    'response with status ' + response.status);
                }

                return cleanResponse(response).then(function(responseToCache) {
                  return cache.put(cacheKey, responseToCache);
                });
              });
            }
          })
        );
      });
    }).then(function() {
      
      // Force the SW to transition from installing -> active state
      return self.skipWaiting();
      
    })
  );
});

self.addEventListener('activate', function(event) {
  var setOfExpectedUrls = new Set(urlsToCacheKeys.values());

  event.waitUntil(
    caches.open(cacheName).then(function(cache) {
      return cache.keys().then(function(existingRequests) {
        return Promise.all(
          existingRequests.map(function(existingRequest) {
            if (!setOfExpectedUrls.has(existingRequest.url)) {
              return cache.delete(existingRequest);
            }
          })
        );
      });
    }).then(function() {
      
      return self.clients.claim();
      
    })
  );
});


self.addEventListener('fetch', function(event) {
  if (event.request.method === 'GET') {
    // Should we call event.respondWith() inside this fetch event handler?
    // This needs to be determined synchronously, which will give other fetch
    // handlers a chance to handle the request if need be.
    var shouldRespond;

    // First, remove all the ignored parameters and hash fragment, and see if we
    // have that URL in our cache. If so, great! shouldRespond will be true.
    var url = stripIgnoredUrlParameters(event.request.url, ignoreUrlParametersMatching);
    shouldRespond = urlsToCacheKeys.has(url);

    // If shouldRespond is false, check again, this time with 'index.html'
    // (or whatever the directoryIndex option is set to) at the end.
    var directoryIndex = 'index.html';
    if (!shouldRespond && directoryIndex) {
      url = addDirectoryIndex(url, directoryIndex);
      shouldRespond = urlsToCacheKeys.has(url);
    }

    // If shouldRespond is still false, check to see if this is a navigation
    // request, and if so, whether the URL matches navigateFallbackWhitelist.
    var navigateFallback = '';
    if (!shouldRespond &&
        navigateFallback &&
        (event.request.mode === 'navigate') &&
        isPathWhitelisted([], event.request.url)) {
      url = new URL(navigateFallback, self.location).toString();
      shouldRespond = urlsToCacheKeys.has(url);
    }

    // If shouldRespond was set to true at any point, then call
    // event.respondWith(), using the appropriate cache key.
    if (shouldRespond) {
      event.respondWith(
        caches.open(cacheName).then(function(cache) {
          return cache.match(urlsToCacheKeys.get(url)).then(function(response) {
            if (response) {
              return response;
            }
            throw Error('The cached response that was expected is missing.');
          });
        }).catch(function(e) {
          // Fall back to just fetch()ing the request if some unexpected error
          // prevented the cached response from being valid.
          console.warn('Couldn\'t serve response for "%s" from cache: %O', event.request.url, e);
          return fetch(event.request);
        })
      );
    }
  }
});







