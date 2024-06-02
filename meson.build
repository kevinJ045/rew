project('rew', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3', 'cpp_std=c++14'])

gtk3 = dependency('gtk+-3.0', method : 'pkg-config')
webkit2gtk = dependency('webkit2gtk-4.0', method : 'pkg-config')
libwebsockets = dependency('libwebsockets', method : 'pkg-config')
jsoncpp = dependency('jsoncpp', method : 'pkg-config')

executable('ui',
           'cpp/ui.cpp',
           install : true,
           dependencies : [gtk3, webkit2gtk, libwebsockets, jsoncpp])
