# Download the binaries from GitHub.
version = "1.4.5"
lib_url = "https://github.com/erikas-taroza/audiotags/releases/download/v#{version}/ios.zip"

`
mkdir Frameworks
cd Frameworks
if [ ! -d ios.zip ]d
then
  curl -L "#{lib_url}" -o ios.zip
  unzip ios.zip -d 'audiotags.xcframework'
fi
cd ..
`

Pod::Spec.new do |s|
  s.name             = 'audiotags'
  s.version          = '1.4.5'
  s.summary          = 'A newFlutter plugin project.'
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
