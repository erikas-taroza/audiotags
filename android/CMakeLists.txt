cmake_minimum_required(VERSION 3.10)

# Download the binaries from GitHub.
set(Version "1.1.0")
set(LibPath "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs")

if(NOT EXISTS "${LibPath}/arm64-v8a/libaudiotags.so")
  file(DOWNLOAD
    "https://github.com/erikas-taroza/audiotags/blob/v${Version}/android/src/main/jniLibs/arm64-v8a/libaudiotags.so?raw=true"
    "${LibPath}/arm64-v8a/libaudiotags.so"
    TLS_VERIFY ON
  )

  file(DOWNLOAD
    "https://github.com/erikas-taroza/audiotags/blob/v${Version}/android/src/main/jniLibs/armeabi-v7a/libaudiotags.so?raw=true"
    "${LibPath}/armeabi-v7a/libaudiotags.so"
    TLS_VERIFY ON
  )

  file(DOWNLOAD
    "https://github.com/erikas-taroza/audiotags/blob/v${Version}/android/src/main/jniLibs/x86/libaudiotags.so?raw=true"
    "${LibPath}/x86/libaudiotags.so"
    TLS_VERIFY ON
  )

  file(DOWNLOAD
    "https://github.com/erikas-taroza/audiotags/blob/v${Version}/android/src/main/jniLibs/x86_64/libaudiotags.so?raw=true"
    "${LibPath}/x86_64/libaudiotags.so"
    TLS_VERIFY ON
  )
endif()