# Download the binary from GitHub.
version = "1.0.1"
lib_url = "https://github.com/erikas-taroza/audiotags/blob/v#{version}/macos/Libs/libaudiotags.a?raw=true"

`
mkdir Libs
cd Libs
if [ ! -f libaudiotags.a ]
then
  curl -L "#{lib_url}" -o libaudiotags.a
fi
cd ..
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
  s.source_files     = 'Classes/**/*'
  s.vendored_libraries = 'Libs/**/*'
  s.dependency 'FlutterMacOS'

  s.platform = :osx, '10.11'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
  s.swift_version = '5.0'
end
