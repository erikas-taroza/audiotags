# Download the binaries from GitHub.
version = "1.0.1"
lib_url = "https://github.com/erikas-taroza/audiotags/blob/v#{version}/ios/Frameworks/audiotags.xcframework"

`
mkdir Frameworks
cd Frameworks
if [ ! -d audiotags.xcframework ]
then
  mkdir audiotags.xcframework
  cd audiotags.xcframework
  mkdir ios-arm64
  mkdir ios-arm64_x86_64-simulator
  curl -L "#{lib_url}/Info.plist?raw=true" -o Info.plist
  curl -L "#{lib_url}/ios-arm64/libaudiotags.a?raw=true" -o ios-arm64/libaudiotags.a
  curl -L "#{lib_url}/ios-arm64_x86_64-simulator/libaudiotags.a?raw=true" -o ios-arm64_x86_64-simulator/libaudiotags.a
fi
cd ../..
`

Pod::Spec.new do |s|
  s.name             = 'audiotags'
  s.version          = '0.0.1'
  s.summary          = 'A new Flutter plugin project.'
  s.description      = <<-DESC
A new Flutter plugin project.
                       DESC
  s.homepage         = 'http://example.com'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Your Company' => 'email@example.com' }
  s.source           = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '9.0'
  s.vendored_frameworks = 'Frameworks/**/*.xcframework'
  s.static_framework = true

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version = '5.0'
end
